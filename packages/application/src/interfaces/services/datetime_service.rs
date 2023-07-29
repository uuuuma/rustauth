use chrono::{DateTime, Utc};
use chrono_tz::Tz;

pub trait DateTimeService {
    fn utc_now(&self) -> DateTime<Utc>;
    fn jst_now(&self) -> DateTime<Tz>;
}
