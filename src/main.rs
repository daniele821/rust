use chrono::{Datelike, NaiveDate};

fn days_in_month(year: u32, month: u32) -> Vec<NaiveDate> {
    let first_day = NaiveDate::from_ymd_opt(year as i32, month, 1).expect("Invalid year or month");

    let num_days = first_day
        .with_day(1)
        .unwrap()
        .with_month(month + 1)
        .unwrap_or_else(|| NaiveDate::from_ymd_opt(year as i32 + 1, 1, 1).unwrap())
        .signed_duration_since(first_day)
        .num_days();

    (0..num_days)
        .map(|i| first_day + chrono::Duration::days(i))
        .collect()
}

fn main() {
    let days = days_in_month(2024, 4); // February 2024 (Leap Year)

    for day in days {
        println!("{}", day);
    }
}
