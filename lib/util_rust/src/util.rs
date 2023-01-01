use time::{OffsetDateTime, PrimitiveDateTime};

#[derive(Debug)]
pub struct Util {}

impl Util {
    pub fn now() -> PrimitiveDateTime {
        let now = OffsetDateTime::now_utc();
        PrimitiveDateTime::new(now.date(), now.time())
    }
}
