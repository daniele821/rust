use chrono::Local;

fn main() {
    let naive = Local::now().naive_local().and_utc().timestamp(); // Remove timezone info

    let datetime = chrono::DateTime::from_timestamp(naive, 0);
    println!("Unix timestamp (UTC): {naive} {datetime:?}");
}

