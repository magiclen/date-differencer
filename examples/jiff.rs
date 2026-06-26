use date_differencer::{add_date_time_diff, date_diff, date_time_diff};
use jiff::{Zoned, civil::date, tz::TimeZone};

fn date_time(
    time_zone: &TimeZone,
    year: i16,
    month: i8,
    day: i8,
    hour: i8,
    minute: i8,
    second: i8,
) -> Zoned {
    date(year, month, day).at(hour, minute, second, 0).to_zoned(time_zone.clone()).unwrap()
}

fn main() {
    // Use the system time zone so this example feels like local date-time code.
    let time_zone = TimeZone::system();

    let a = date_time(&time_zone, 2022, 4, 6, 0, 0, 0);
    let b = date_time(&time_zone, 2023, 6, 9, 1, 0, 0);

    println!("{:?}", date_diff(a.clone(), b.clone()));
    /*
    {
        "years": 1,
        "months": 2,
        "days": 3
    }
    */

    println!("{:?}", date_time_diff(a.clone(), b.clone()));
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

    println!("{}", add_date_time_diff(a.clone(), &date_time_diff(a.clone(), b.clone())).unwrap()); // the same as b

    let a = date_time(&time_zone, 2020, 2, 27, 0, 0, 0);
    let b = date_time(&time_zone, 2021, 3, 1, 0, 0, 0);

    println!("{:?}", date_diff(a.clone(), b.clone()));
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
