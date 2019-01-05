use std::net::ToSocketAddrs;
use failure::Error;
use futures::future::{Shared, Future};
use futures::stream::{Stream};
use tokio::net::TcpStream;
use lapin_futures::client::ConnectionOptions;
use lapin_futures::channel::{BasicPublishOptions, BasicProperties};

pub fn run<F, S>(
        teardown: Shared<F>,
        addr: &str,
        exchange: &str,
        sample_stream: S
    ) -> Box<Future<Item = (), Error = Error> + Send>
        where F: Future<Item = Option<i32>, Error = std::io::Error> + Send + 'static,
              S: Stream<Item = Vec<u8>, Error = Error> + Send + 'static {
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
            .for_each(move |sample| {
                channel
                    .basic_publish(
                        &exchange,
                        "sensor",
                        sample,
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
