use chrono::NaiveDate;

fn main() {
    let year = 2024;
    for (m, d) in (1..=12).map(|m| {
        (
            m,
            if m == 12 {
                NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap()
            } else {
                NaiveDate::from_ymd_opt(year, m + 1, 1).unwrap()
            }
            .signed_duration_since(NaiveDate::from_ymd_opt(year, m, 1).unwrap())
            .num_days(),
        )
    }) {
        println!("days {d} in month {m}");
    }
}
