use chrono::{Local, TimeZone, Utc};

fn main() {
    let now = Local::now(); // Get current local datetime
    let naive = now.naive_local(); // Remove timezone info
    let utc = Utc.from_utc_datetime(&naive); // Convert to UTC
    let timestamp = utc.timestamp(); // Get Unix timestamp (seconds only)

    println!("Unix timestamp (UTC, no milliseconds): {}", timestamp);
}
