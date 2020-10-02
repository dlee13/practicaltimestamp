use super::UnixTimestamp;

#[derive(Debug)]
pub enum TimestampResult {
    TimestampOk(UnixTimestamp),
    OverflowErr(i64),
}

impl TimestampResult {
    #[track_caller] // MSRV 1.46
    pub fn expect(self, msg: &'static str) -> UnixTimestamp {
        match self {
            Self::TimestampOk(timestamp) => timestamp,
            _ => panic!(msg),
        }
    }

    pub const fn ok(self) -> Option<UnixTimestamp> {
        match self {
            Self::TimestampOk(timestamp) => Some(timestamp),
            _ => None,
        }
    }

    pub const fn unwrap(self) -> UnixTimestamp {
        match self {
            Self::TimestampOk(timestamp) => timestamp,
            Self::OverflowErr(value) => {
                // Overflowed timestamp saturates when unwrapped
                if value < UnixTimestamp::MIN.unix_timestamp() {
                    UnixTimestamp::MIN
                } else {
                    UnixTimestamp::MAX
                }
            },
        }
    }
}
