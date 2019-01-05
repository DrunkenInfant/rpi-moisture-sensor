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
        let json = json!({
            "sensor_type": self.sample_type,
            "sensor_id": self.sensor_id,
            "timestamp": timestamp.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as u64,
            "value": sample
        });
        serde_json::to_vec(&json).unwrap()
    }
}
