use chrono::{Local, TimeZone};

fn main() {
    // Get current time as Unix timestamp
    let now = Local::now().timestamp();

    // Convert timestamp back to datetime
    let datetime = Local.timestamp_opt(now, 0).unwrap();

    // Format as 'YYYY/MM/DD hh:mm'
    let formatted_time = datetime.format("%Y/%m/%d %H:%M").to_string();

    // Print result
    println!("time:{now} | {formatted_time}");
}
