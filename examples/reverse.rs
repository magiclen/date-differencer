use chrono::prelude::*;

use date_differencer::date_diff;

fn main() {
    let a = Local.with_ymd_and_hms(2020, 2, 27, 0, 0, 0).unwrap();
    let b = Local.with_ymd_and_hms(2021, 3, 1, 0, 0, 0).unwrap();

    println!("{:?}", date_diff(a, b));
    /*
    {
        "years": 1,
        "months": 0,
        "days": 2
    }

    Explanation:
        1. 2020-02-27 + 1 year -> 2021-02-27
        2. 2021-02-27 + 2 days -> 2021-03-01 (2021-02 has 28 days)
    */

    println!("{:?}", date_diff(b, a));
    /*
    {
        "years": -1,
        "months": 0,
        "days": -3
    }

    Explanation:
        1. 2021-03-01 - 1 year -> 2020-03-01
        2. 2020-03-01 - 3 days -> 2020-02-27 (2020-02 has 29 days)
    */
}
