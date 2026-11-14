use std::time::Duration;
use time::PlainDateTime as DateTime;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start + Duration::from_secs(1000_000_000)
}
