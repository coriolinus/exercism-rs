extern crate chrono;
use chrono::*;

/// Return a DateTime 1 Gigasecond (10e9) seconds after the one given.
pub fn after<Tz: TimeZone>(start: DateTime<Tz>) -> DateTime<Tz> {
    // it would be nice if this could be a const, but that doesn't work because you can't define
    // a const using a function (like Duration::seconds). Tried using lazy_static to make it
    // a static, but that gave me type errors, so whatever: it's just hard-coded in here
    start + Duration::seconds(1000000000)
}
