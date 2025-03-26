use chrono::{Local, NaiveDateTime, TimeZone};

fn main() {
    // Get current time as Unix timestamp
    let now = Local::now().timestamp();

    // Convert timestamp back to datetime
    let datetime = Local.timestamp_opt(now, 0).unwrap();

    // Format as 'YYYY/MM/DD hh:mm'
    let formatted_time = datetime.format("%Y/%m/%d %H:%M:%S").to_string();

    // Print result
    println!("time:{now} | {formatted_time}");

    let naive_dt = NaiveDateTime::parse_from_str(&formatted_time, "%Y/%m/%d %H:%M:%S").unwrap();

    // Convert to Unix timestamp using the local timezone
    let unix_time = Local.from_local_datetime(&naive_dt).unwrap().timestamp();

    // Print Unix timestamp
    println!("{unix_time}");

    assert_eq!(unix_time, now);
}
