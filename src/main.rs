/// Find the Friday 13ths in a given year range.

use chrono::prelude::*;

fn main() {
    let start_year = 2015;
    let end_year = 2025;

    let mut count = 0;
    for year in start_year..=end_year {
        for month in 1..=12 {
            let dt = Utc.ymd(year, month, 13);
            if dt.weekday() == Weekday::Fri {
                println!("{}\t{}", dt.year(), dt.format("%A, %d %B %Y"));
                count += 1;
            }
        }
        println!("----");
    }

    println!("Between {} and {} this occurs {} times.", start_year, end_year, count);
}
