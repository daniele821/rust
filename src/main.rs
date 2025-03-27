use chrono::{DateTime, Local, NaiveDateTime};

fn main() {
    let naive = Local::now().naive_local().and_utc().timestamp(); // Remove timezone info

    let datetime = chrono::DateTime::from_timestamp(naive, 0);
    println!("Unix timestamp (UTC): {naive} {datetime:?}");

    let fmt = DateTime::from_timestamp(naive, 0)
        .unwrap()
        .format("%Y/%m/%d %H:%M")
        .to_string();
    println!("{fmt:?}");
}
