use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime};
use futures::{Async, Poll};
use futures::stream::Stream;
use tokio_timer::Interval;
use crate::gpio::Gpio;
use crate::sensor::Sensor;

pub struct SensorSampler<S: Sensor> {
    sensor: S,
    gpio: Arc<Mutex<Gpio>>,
    timer: Interval,
}

impl<S: Sensor> std::ops::Drop for SensorSampler<S> {
    fn drop(&mut self) {
        let _ = self.sensor.clear(&mut self.gpio.lock().unwrap());
    }
}

impl<S: Sensor> SensorSampler<S> {
    pub fn new(sensor: S, gpio: Arc<Mutex<Gpio>>, interval: u64) -> Self {
        let timer = Interval::new(Instant::now(), Duration::from_secs(interval));
        SensorSampler { sensor, gpio, timer }
    }
}

impl<S: Sensor> Stream for SensorSampler<S> {
    type Item = (SystemTime, S::Value);
    type Error = S::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        // TODO handler errors instead of unwrap
        match self.timer.poll() {
            Ok(Async::Ready(Some(_))) => {
                let sample = { self.sensor.read(&mut self.gpio.lock().unwrap()).unwrap() };
                Ok(Async::Ready(Some((SystemTime::now(), sample))))
            },
            Ok(Async::Ready(None)) => Ok(Async::Ready(None)),
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Err(err) => {
                println!("Error on timer in sampler {}", err);
                Ok(Async::Ready(None))
            }
        }
    }
}
