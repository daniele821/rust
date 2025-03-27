use chrono::Local;

fn main() {
    let naive = Local::now().naive_local().and_utc().timestamp(); // Remove timezone info

    println!("Unix timestamp (UTC): {naive}");
}

