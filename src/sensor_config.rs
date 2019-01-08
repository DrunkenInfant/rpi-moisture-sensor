use toml::Value;
use failure::Error as FailureError;
use crate::sensor::Sensor;
use crate::moist_sensor::MoistSensor;
use crate::sample_formatter::SampleFormatter;

#[derive(Debug)]
pub struct Error {
    key: String,
    cause: String
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "Sensor configuration error at [{}]: {}", self.key, self.cause)
    }
}

impl std::error::Error for Error {
}

#[derive(Debug)]
pub struct SensorConfig {
    pub id: String,
    pub sensor_type: String,
    pub pwr: i64,
    pub val: i64
}

#[derive(Debug)]
pub struct SensorsConfig {
    sensors: Vec<SensorConfig>
}

pub fn from_toml(toml_str: &str) -> Result<SensorsConfig, FailureError> {
    let value = toml_str.parse::<Value>()?;
    Ok(SensorsConfig::from_toml(&value)?)
}

impl SensorsConfig {
    pub fn from_toml(toml: &Value) -> Result<SensorsConfig, FailureError> {
        get_key_as(toml, "sensors", Value::as_table, "", "table/object")?
            .iter()
            .map(|(id, toml)| {
                SensorConfig::from_toml(id, toml)
            })
            .collect::<Result<Vec<SensorConfig>, FailureError>>()
            .map(|sensors| {
                SensorsConfig { sensors }
            })
    }

    pub fn validate(&self) -> Result<(), FailureError> {
        match self.sensors.as_slice() {
            [sensor] => Ok(sensor.validate()?),
            _ => Err(FailureError::from(Error { key: "sensors".to_string(), cause: "Only one sensor supported.".to_string() }))
        }
    }

    pub fn initialize(&self) -> Result<(impl Sensor, SampleFormatter), FailureError> {
        match self.sensors.as_slice() {
            [sensor] => Ok(sensor.initialize()?),
            _ => Err(FailureError::from(Error { key: "sensors".to_string(), cause: "Only one sensor supported.".to_string() }))
        }
    }
}

impl SensorConfig {
    pub fn from_toml(id: &str, conf: &Value) -> Result<SensorConfig, FailureError> {
        let parent_key = &format!("sensors.{}", id);
        let sensor_type = get_key_as(conf, "sensor_type", |toml| { toml.as_str() }, parent_key, "string")?;
        let pwr = get_key_as(conf, "pwr_pin", |toml| { toml.as_integer() }, parent_key, "integer")?;
        let val = get_key_as(conf, "val_pin", |toml| { toml.as_integer() }, parent_key, "integer")?;

        Ok(SensorConfig {
            id: id.to_string(),
            sensor_type: sensor_type.to_string(),
            pwr: pwr,
            val: val
        })
    }

    pub fn validate(&self) -> Result<(), FailureError> {
        validate_pin(self.pwr)?;
        validate_pin(self.val)?;
        Ok(())
    }

    pub fn initialize(&self) -> Result<(impl Sensor, SampleFormatter), FailureError> {
        Ok((MoistSensor::new(self.pwr as u8, self.val as u8), SampleFormatter::new(self.id.clone(), self.sensor_type.clone())))
    }
}

fn validate_pin(pin: i64) -> Result<(), FailureError> {
    match pin {
        4 | 5 | 6 | 13 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 => Ok(()),
        _ => Err(FailureError::from(Error { key: "pin".to_string(), cause: format!("Not a valid bcm pin: {}", pin).to_string() }))
    }
}

fn get_key_as<'a, F, V>(value: &'a Value, key: &str, f: F, scope: &str, expected_type: &str) -> Result<V, FailureError>
    where F: FnOnce(&'a Value) -> Option<V> {
    match value.get(key) {
        Some(v) => f(v).ok_or(FailureError::from(Error {
                key: format!("{}.{}", scope, key).to_string(),
                cause: format!("Is not valid type, expected '{}' but found '{}'", expected_type, v.type_str()).to_string()
            })),
        None => Err(FailureError::from(Error {
            key: format!("{}.{}", scope, key).to_string(),
            cause: "Was expected but not found".to_string()
        }))
    }
}
