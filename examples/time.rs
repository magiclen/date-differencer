use date_differencer::{add_date_time_diff, date_diff, date_time_diff};
use time::{Date, Month, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset};

fn date_time(
    offset: UtcOffset,
    year: i32,
    month: Month,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
) -> OffsetDateTime {
    PrimitiveDateTime::new(
        Date::from_calendar_date(year, month, day).unwrap(),
        Time::from_hms(hour, minute, second).unwrap(),
    )
    .assume_offset(offset)
}

fn main() {
    let offset = UtcOffset::current_local_offset().unwrap();

    let a = date_time(offset, 2022, Month::April, 6, 0, 0, 0);
    let b = date_time(offset, 2023, Month::June, 9, 1, 0, 0);

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

    let a = date_time(offset, 2020, Month::February, 27, 0, 0, 0);
    let b = date_time(offset, 2021, Month::March, 1, 0, 0, 0);

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
