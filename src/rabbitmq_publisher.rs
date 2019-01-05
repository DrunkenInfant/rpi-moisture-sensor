use std::ops::Add;
use std::net::ToSocketAddrs;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use failure::Error;
use futures::future::{Shared, Future, Loop};
use tokio::net::TcpStream;
use tokio::timer::Delay;
use lapin_futures::client::ConnectionOptions;
use lapin_futures::channel::{BasicPublishOptions, BasicProperties};

use crate::gpio::Gpio;
use crate::moist_sensor::MoistSensor;

fn u32_to_bytes(x:u32) -> [u8;4] {
    let b1 : u8 = ((x >> 24) & 0xff) as u8;
    let b2 : u8 = ((x >> 16) & 0xff) as u8;
    let b3 : u8 = ((x >> 8) & 0xff) as u8;
    let b4 : u8 = (x & 0xff) as u8;
    return [b1, b2, b3, b4]
}

pub fn run<F>(
        teardown: Shared<F>,
        addr: &str,
        exchange: &str,
        sensor_interval: u64,
        moist: MoistSensor,
        gp: Arc<Mutex<Gpio>>) -> Box<Future<Item = u64, Error = Error> + Send> where F: Future<Item = Option<i32>, Error = std::io::Error> + Send + 'static {
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
        futures::future::loop_fn(0_u64, move |count| {
            let teardown = teardown.clone();
            let val = { moist.read(&mut gp.lock().unwrap()).unwrap() };
            channel
                .basic_publish(
                    &exchange,
                    "sensor",
                    u32_to_bytes(val).to_vec(),
                    BasicPublishOptions::default(),
                    BasicProperties::default()
                )
                .map_err(Error::from)
                .and_then(move |_| {
                    let delay = Delay::new(Instant::now().add(Duration::from_secs(sensor_interval)))
                        .map(|_| None)
                        .map_err(Error::from);
                    teardown
                        .then(|_| -> Result<Option<()>, Error> { Ok(Some(())) })
                        .select(delay)
                        .map(|(v, _)| v)
                        .map_err(|(e, _)| e)
                })
                .and_then(move |res: Option<()>| -> Result<Loop<u64, u64>, Error> {
                    match res {
                        Some(_) => Ok(Loop::Break(count + 1)),
                        None => Ok(Loop::Continue(count + 1))
                    }
                })
        })
    })
    .map_err(Error::from);

    std::boxed::Box::new(fut)
}
