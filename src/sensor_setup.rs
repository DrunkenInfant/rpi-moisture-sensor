use std::sync::{Arc, Mutex};
use failure::Error as FailureError;
use futures::stream::{Stream};
use crate::gpio::{Gpio};
use crate::moist_sensor::MoistSensor;
use crate::sensor_config::{SensorsConfig, SensorConfig};
use crate::sensor_sampler::SensorSampler;
use crate::sample_formatter::SampleFormatter;

pub fn setup(config: &SensorsConfig, gpio: Arc<Mutex<Gpio>>)
    -> Result<Box<Stream<Item = Vec<u8>, Error = FailureError> + Send>, FailureError> {
    config.sensors
        .iter()
        .fold(
            Ok(Box::new(futures::stream::empty::<Vec<u8>, failure::Error>())),
            |combined_stream, sc| {
                match combined_stream {
                    Ok(c) => match setup_one(sc, gpio.clone()) {
                        Ok(s) => Ok(Box::new(c.select(s))),
                        e => e
                    },
                    e => e
                }
            }
        )
}

fn setup_one(config: &SensorConfig, gpio: Arc<Mutex<Gpio>>)
    -> Result<Box<Stream<Item = Vec<u8>, Error = FailureError> + Send>, FailureError> {
    let sensor = MoistSensor::new(config.pwr as u8, config.val as u8, config.pwr_wait);
    sensor.init(&mut gpio.lock().unwrap()).unwrap();
    let formatter = SampleFormatter::new(config.id.clone(), config.sensor_type.clone());
    let sampler = SensorSampler::new(
        sensor,
        gpio,
        config.interval
    );

    Ok(Box::new(sampler
        .map(move |sample| formatter.format(&sample))
        .map_err(failure::Error::from)
    ))
}
