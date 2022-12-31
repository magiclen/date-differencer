Date Differencer
====================

[![CI](https://github.com/magiclen/date-differencer/actions/workflows/ci.yml/badge.svg)](https://github.com/magiclen/date-differencer/actions/workflows/ci.yml)

Calculate the time interval between two `DateTime` instances and output the result in years plus months plus days plus hours plus minutes plus seconds plus nanoseconds (instead of representing the same duration in different units). This library is useful for lifespan check and age calculation.

## Usage

```rust
use chrono::prelude::*;

use date_differencer::{date_diff, date_time_diff, add_date_time_diff};

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
```

This library can handle leap years and odd/even number of days in a month correctly. The result of following code is a bit confusing but reasonable.

```rust
use chrono::prelude::*;

use date_differencer::date_diff;

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
```

## Crates.io

https://crates.io/crates/date-differencer

## Documentation

https://docs.rs/date-differencer

## License

[MIT](LICENSE)