use chrono::prelude::*;

use date_differencer::{add_date_time_diff, date_diff, date_time_diff};

fn main() {
    let a = Local.with_ymd_and_hms(2022, 4, 6, 0, 0, 0).unwrap();
    let b = Local.with_ymd_and_hms(2023, 6, 9, 1, 0, 0).unwrap();

    println!("{:?}", date_diff(a, b));
    /*
    {
        "years": 1,
        "months": 2,
        "days": 3
    }
    */

    println!("{:?}", date_time_diff(a, b));
    /*
    {
        "years": 1,
        "months": 2,
        "days": 3,
        "hours": 1,
        "minutes": 0,
        "seconds": 0,
        "nanoseconds": 0
    }
    */

    println!("{}", add_date_time_diff(a, &date_time_diff(a, b)).unwrap()); // the same as b
}
