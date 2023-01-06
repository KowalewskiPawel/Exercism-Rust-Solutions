use time::PrimitiveDateTime as DateTime;
use time::Duration;
// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    const GIGASECOND: i64 = 1_000_000_000;
    let duration = Duration::seconds(GIGASECOND);
    start + duration
}