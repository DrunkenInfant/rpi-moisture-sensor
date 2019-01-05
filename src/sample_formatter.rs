use std::time::SystemTime;

pub struct SampleFormatter {
    sensor_id: String,
    sample_type: String
}

impl SampleFormatter {
    pub fn new(sensor_id: String, sample_type: String) -> Self {
        Self { sensor_id, sample_type }
    }

    pub fn format(&self, (timestamp, sample): &(SystemTime, u32)) -> Vec<u8> {
        let msg = format!(
            "{}:{}:{}:{}",
            self.sample_type,
            self.sensor_id,
            timestamp.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis(),
            sample
        );
        msg.as_bytes().to_vec()
    }
}
