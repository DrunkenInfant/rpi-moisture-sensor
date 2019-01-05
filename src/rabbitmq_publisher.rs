use std::net::ToSocketAddrs;
use std::time::SystemTime;
use failure::Error;
use futures::future::{Shared, Future};
use futures::stream::{Stream};
use tokio::net::TcpStream;
use lapin_futures::client::ConnectionOptions;
use lapin_futures::channel::{BasicPublishOptions, BasicProperties};

fn u32_to_bytes(x:u32) -> [u8;4] {
    let b1 : u8 = ((x >> 24) & 0xff) as u8;
    let b2 : u8 = ((x >> 16) & 0xff) as u8;
    let b3 : u8 = ((x >> 8) & 0xff) as u8;
    let b4 : u8 = (x & 0xff) as u8;
    return [b1, b2, b3, b4]
}

pub fn run<F, S>(
        teardown: Shared<F>,
        addr: &str,
        exchange: &str,
        sample_stream: S
    ) -> Box<Future<Item = (), Error = Error> + Send>
        where F: Future<Item = Option<i32>, Error = std::io::Error> + Send + 'static,
              S: Stream<Item = (SystemTime, u32), Error = Error> + Send + 'static {
    let exchange = exchange.to_string();
    let teardown = teardown.clone();

    let fut = TcpStream::connect(&addr.to_socket_addrs().unwrap().next().unwrap()).map_err(Error::from).and_then(|stream| {
        lapin_futures::client::Client::connect(stream, ConnectionOptions {
            frame_max: 65535,
            ..Default::default()
        }).map_err(Error::from)
    }).and_then(|(client, _ /* heartbeat */)| {
        client.create_channel().map_err(Error::from)
    }).and_then(move |channel| {
        let teardown = teardown
            .then(|_| -> Result<(), Error> { Ok(()) });
        let stream = sample_stream
            .for_each(move |(_timestamp, sample)| {
                channel
                    .basic_publish(
                        &exchange,
                        "sensor",
                        u32_to_bytes(sample).to_vec(),
                        BasicPublishOptions::default(),
                        BasicProperties::default()
                    )
                    .map(|_| ())
                    .map_err(Error::from)
            });
        teardown
            .select(stream)
            .map(|(v, _)| v)
            .map_err(|(e, _)| e)
    })
    .map_err(Error::from);

    std::boxed::Box::new(fut)
}
