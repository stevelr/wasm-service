use std::fmt;

/// Time in UTC, with conversions to/from u64 and rfc2822
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct HttpDate(u64);

/// Convert HttpDate to printable string in rfc2822 format
impl fmt::Display for HttpDate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use chrono::{DateTime, NaiveDateTime, Utc};

        let dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(self.0 as i64, 0), Utc);
        fmt::Display::fmt(&dt.to_rfc2822(), f)
    }
}

/// Convert u64 timestamp (seconds since EPOCH in UTC) to HttpDate
impl From<u64> for HttpDate {
    fn from(utc_sec: u64) -> HttpDate {
        HttpDate(utc_sec)
    }
}

/// Convert i64 timestamp (seconds since EPOCH in UTC) to HttpDate
impl From<i64> for HttpDate {
    fn from(utc_sec: i64) -> HttpDate {
        HttpDate(utc_sec as u64)
    }
}

impl std::str::FromStr for HttpDate {
    type Err = chrono::format::ParseError;

    /// Parse string to HttpDate
    fn from_str(s: &str) -> Result<HttpDate, Self::Err> {
        let utc_sec = chrono::DateTime::parse_from_rfc2822(s).map(|dt| dt.timestamp() as u64)?;
        Ok(HttpDate(utc_sec))
    }
}

impl HttpDate {
    /// Convert HttpDate to i64 timestamp (seconds since EPOCH in UTC)
    pub fn timestamp(&self) -> u64 {
        self.0
    }
}
