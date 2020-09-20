use super::Timestamp;

#[derive(Debug)]
pub enum TimestampResult {
    TimestampOk(Timestamp),
    OverflowErr(i64),
}

impl TimestampResult {
    #[track_caller] // MSRV 1.46
    pub fn expect(self, msg: &'static str) -> Timestamp {
        match self {
            Self::TimestampOk(timestamp) => timestamp,
            _ => panic!(msg),
        }
    }

    pub const fn ok(self) -> Option<Timestamp> {
        match self {
            Self::TimestampOk(timestamp) => Some(timestamp),
            _ => None,
        }
    }

    pub const fn unwrap(self) -> Timestamp {
        match self {
            Self::TimestampOk(timestamp) => timestamp,
            Self::OverflowErr(value) => {
                // Overflowed timestamp saturates when unwrapped
                if value < Timestamp::MIN.unix_timestamp() {
                    Timestamp::MIN
                } else {
                    Timestamp::MAX
                }
            },
        }
    }
}
