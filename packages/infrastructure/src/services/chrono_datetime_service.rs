use chrono::{DateTime, Utc};
use chrono_tz::{Asia::Tokyo, Tz};

use application::interfaces::services::datetime_service::DateTimeService;

pub struct ChronoDateTimeService;

impl ChronoDateTimeService {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ChronoDateTimeService {
    fn default() -> Self {
        Self::new()
    }
}

impl DateTimeService for ChronoDateTimeService {
    fn utc_now(&self) -> DateTime<Utc> {
        Utc::now()
    }
    fn jst_now(&self) -> DateTime<Tz> {
        Utc::now().with_timezone(&Tokyo)
    }
}
