use std::time::{Duration, SystemTime};
use super::UnixTimestamp;

pub fn system_time_now() -> UnixTimestamp {
    SystemTime::now().into()
}

impl From<SystemTime> for UnixTimestamp {
    fn from(system_time: SystemTime) -> Self {
        let timestamp = match system_time.duration_since(SystemTime::UNIX_EPOCH) {
            Ok(duration) => duration.as_secs() as i64,
            Err(_) => panic!("SystemTime before UNIX EPOCH"),
        };
        Self::from_unix_timestamp(timestamp).expect("SystemTime out of range of Timestamp")
    }
}

impl From<UnixTimestamp> for SystemTime {
    fn from(timestamp: UnixTimestamp) -> Self {
        let timestamp = timestamp.unix_timestamp() as u64;
        Self::UNIX_EPOCH + Duration::from_secs(timestamp)
    }
}
