use sea_orm::prelude::{TimeDateTime, TimeDateTimeWithTimeZone};

#[derive(Debug)]
pub struct Util {}

impl Util {
    pub fn now() -> TimeDateTime {
        let now = TimeDateTimeWithTimeZone::now_utc();
        TimeDateTime::new(now.date(), now.time())
    }
}
