#[cfg(feature = "chrono")]
mod chrono_support;
#[cfg(feature = "jiff")]
mod jiff_support;
#[cfg(feature = "time")]
mod time_support;

use core::cmp::Ordering;

use super::constants::*;

#[derive(Debug)]
struct TimeDiffResult {
    pub(crate) hours:       i32,
    pub(crate) minutes:     i32,
    pub(crate) seconds:     i32,
    pub(crate) nanoseconds: i32,
}

/// The result of the `date_diff` function.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub struct DateDiffResult {
    pub years:  i32,
    pub months: i32,
    pub days:   i32,
}

impl DateDiffResult {
    #[doc(hidden)]
    #[inline]
    pub fn into_neg(mut self) -> Self {
        self.years *= -1;
        self.months *= -1;
        self.days *= -1;

        self
    }
}

/// The date and time fields of a date-time value.
#[derive(Debug, Clone, Copy)]
pub(crate) struct DateTimeFields {
    pub(crate) year:       i32,
    pub(crate) month:      u8,
    pub(crate) day:        u8,
    pub(crate) hour:       u8,
    pub(crate) minute:     u8,
    pub(crate) second:     u8,
    pub(crate) nanosecond: u32,
}

impl DateTimeFields {
    #[inline]
    pub(crate) fn from_parts(date_time: &impl DateTimeParts) -> Self {
        let (year, month, day, hour, minute, second, nanosecond) = date_time.all_parts();

        Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
            nanosecond,
        }
    }
}

/// The wall-clock date and time of a date-time value.
// The field order matters because `Ord` is derived.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
struct WallClock {
    year:               i32,
    month:              u8,
    day:                u8,
    nanoseconds_of_day: u64,
}

impl WallClock {
    #[inline]
    fn from_parts(date_time: &impl DateTimeParts) -> Self {
        let fields = DateTimeFields::from_parts(date_time);

        Self {
            year:               fields.year,
            month:              fields.month,
            day:                fields.day,
            nanoseconds_of_day: (fields.hour as u64 * HOUR_NANOSECONDS)
                + (fields.minute as u64 * MINUTE_NANOSECONDS)
                + (fields.second as u64 * SECOND_NANOSECONDS)
                + fields.nanosecond as u64,
        }
    }
}

#[derive(Debug)]
struct _DateDiffResult {
    pub(crate) earlier_nanoseconds_of_day: u64,
    pub(crate) later_nanoseconds_of_day:   u64,
    pub(crate) result:                     DateDiffResult,
}

/// The result of the `date_time_diff` function.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub struct DateTimeDiffResult {
    pub years:       i32,
    pub months:      i32,
    pub days:        i32,
    pub hours:       i32,
    pub minutes:     i32,
    pub seconds:     i32,
    pub nanoseconds: i32,
}

impl DateTimeDiffResult {
    #[doc(hidden)]
    #[inline]
    pub fn into_neg(mut self) -> Self {
        self.years *= -1;
        self.months *= -1;
        self.days *= -1;
        self.hours *= -1;
        self.minutes *= -1;
        self.seconds *= -1;
        self.nanoseconds *= -1;

        self
    }
}

impl From<DateDiffResult> for DateTimeDiffResult {
    #[inline]
    fn from(value: DateDiffResult) -> Self {
        DateTimeDiffResult {
            years: value.years,
            months: value.months,
            days: value.days,
            ..DateTimeDiffResult::default()
        }
    }
}

impl From<DateTimeDiffResult> for DateDiffResult {
    #[inline]
    fn from(value: DateTimeDiffResult) -> Self {
        DateDiffResult {
            years: value.years, months: value.months, days: value.days
        }
    }
}

/// A trait to represent a date-time difference with multiple units.
pub trait DateTimeDiff {
    #[inline]
    fn years(&self) -> i32 {
        0
    }

    #[inline]
    fn months(&self) -> i32 {
        0
    }

    #[inline]
    fn days(&self) -> i32 {
        0
    }

    #[inline]
    fn hours(&self) -> i32 {
        0
    }

    #[inline]
    fn minutes(&self) -> i32 {
        0
    }

    #[inline]
    fn seconds(&self) -> i32 {
        0
    }

    #[inline]
    fn nanoseconds(&self) -> i32 {
        0
    }
}

/// A trait to expose the date and time fields used by this crate.
///
/// The fields describe the wall-clock date and time of the value.
/// Each method must return a value in the range written in its documentation, otherwise the functions of this crate may panic.
///
/// This trait is also implemented for `chrono` types.
/// If `chrono::Datelike` or `chrono::Timelike` is in scope at the same time (for example, by `use chrono::prelude::*;` and `use date_differencer::*;`), calling a method such as `year()` on a `chrono` value is ambiguous and does not compile.
/// To avoid this, import only the items you need from this crate (for example, `use date_differencer::{date_diff, date_time_diff};`), or call the method with its full path (for example, `Datelike::year(&date_time)`).
pub trait DateTimeParts: Ord {
    /// Returns the year.
    fn year(&self) -> i32;

    /// Returns the month, from `1` to `12`.
    fn month(&self) -> u8;

    /// Returns the day of the month, from `1` to the number of days in the month.
    fn day(&self) -> u8;

    /// Returns the hour, from `0` to `23`.
    fn hour(&self) -> u8;

    /// Returns the minute, from `0` to `59`.
    fn minute(&self) -> u8;

    /// Returns the second, from `0` to `59`.
    fn second(&self) -> u8;

    /// Returns the nanosecond, from `0` to `999_999_999`.
    fn nanosecond(&self) -> u32;

    /// Returns all fields at once, in the order of year, month, day, hour, minute, second and nanosecond.
    #[doc(hidden)]
    #[inline]
    fn all_parts(&self) -> (i32, u8, u8, u8, u8, u8, u32) {
        (
            self.year(),
            self.month(),
            self.day(),
            self.hour(),
            self.minute(),
            self.second(),
            self.nanosecond(),
        )
    }

    /// Converts `other` to the time zone of `self` without changing the instant it represents.
    #[doc(hidden)]
    #[inline]
    fn to_same_time_zone(&self, other: Self) -> Self
    where
        Self: Sized, {
        other
    }
}

impl DateTimeDiff for DateDiffResult {
    #[inline]
    fn years(&self) -> i32 {
        self.years
    }

    #[inline]
    fn months(&self) -> i32 {
        self.months
    }

    #[inline]
    fn days(&self) -> i32 {
        self.days
    }
}

impl DateTimeDiff for DateTimeDiffResult {
    #[inline]
    fn years(&self) -> i32 {
        self.years
    }

    #[inline]
    fn months(&self) -> i32 {
        self.months
    }

    #[inline]
    fn days(&self) -> i32 {
        self.days
    }

    #[inline]
    fn hours(&self) -> i32 {
        self.hours
    }

    #[inline]
    fn minutes(&self) -> i32 {
        self.minutes
    }

    #[inline]
    fn seconds(&self) -> i32 {
        self.seconds
    }

    #[inline]
    fn nanoseconds(&self) -> i32 {
        self.nanoseconds
    }
}

#[inline]
const fn _nanoseconds_to_units(mut nanoseconds: u64) -> TimeDiffResult {
    let h = nanoseconds / HOUR_NANOSECONDS;
    nanoseconds -= h * HOUR_NANOSECONDS;

    let m = nanoseconds / MINUTE_NANOSECONDS;
    nanoseconds -= m * MINUTE_NANOSECONDS;

    let s = nanoseconds / SECOND_NANOSECONDS;
    nanoseconds -= s * SECOND_NANOSECONDS;

    TimeDiffResult {
        hours:       h as i32,
        minutes:     m as i32,
        seconds:     s as i32,
        nanoseconds: nanoseconds as i32,
    }
}

#[inline]
const fn _time_diff(
    earlier_nanoseconds_of_day: u64,
    later_nanoseconds_of_day: u64,
) -> TimeDiffResult {
    let nanoseconds = if later_nanoseconds_of_day >= earlier_nanoseconds_of_day {
        later_nanoseconds_of_day - earlier_nanoseconds_of_day
    } else {
        DAY_NANOSECONDS + later_nanoseconds_of_day - earlier_nanoseconds_of_day
    };

    _nanoseconds_to_units(nanoseconds)
}

fn _date_diff(earlier: &WallClock, later: &WallClock, start_from_later: bool) -> _DateDiffResult {
    let mut earlier_year = earlier.year;
    let mut earlier_month = earlier.month;
    let mut earlier_date = earlier.day;

    let mut later_year = later.year;
    let mut later_month = later.month;
    let mut later_date = later.day;

    let later_nanoseconds_of_day = later.nanoseconds_of_day;
    let earlier_nanoseconds_of_day = earlier.nanoseconds_of_day;

    let years: i32;
    let months: i32;
    let days: i32;

    if later_nanoseconds_of_day < earlier_nanoseconds_of_day {
        // e.g. 12:00 to 11:59

        if start_from_later {
            // increase a day from the earlier date

            if earlier_date < year_helper::get_days_in_month(earlier_year, earlier_month).unwrap() {
                // e.g. 2020-01-12 12:00 to 2022-02-15 11:59

                earlier_date += 1;
            } else if earlier_month < 12 {
                // e.g. 2020-01-31 12:00 to 2022-02-15 11:59

                earlier_month += 1;
                earlier_date = 1;
            } else {
                // e.g. 2020-12-31 12:00 to 2022-02-15 11:59

                earlier_year += 1;
                earlier_month = 1;
                earlier_date = 1;
            }
        } else {
            // decrease a day from the later date

            if later_date > 1 {
                // e.g. 2020-01-12 12:00 to 2022-02-15 11:59

                later_date -= 1;
            } else if later_month > 1 {
                // e.g. 2020-01-12 12:00 to 2022-02-01 11:59

                later_month -= 1;
                later_date = year_helper::get_days_in_month(later_year, later_month).unwrap();
            } else {
                // e.g. 2020-01-12 12:00 to 2022-01-01 11:59

                later_year -= 1;
                later_month = 12;
                later_date = 31;
            }
        }
    }

    let year_diff = later_year - earlier_year;
    let month_diff = later_month as i32 - earlier_month as i32;

    match month_diff.cmp(&0) {
        Ordering::Greater => {
            // e.g. 2010-01 to 2010-03

            years = year_diff;

            if later_date >= earlier_date {
                // e.g. 2010-01-02 to 2010-03-04

                months = month_diff;
            } else {
                // e.g. 2010-01-02 to 2010-03-01

                months = month_diff - 1;
            }
        },
        Ordering::Less => {
            // e.g. 2009-11 to 2010-03

            years = year_diff - 1;

            if later_date >= earlier_date {
                // e.g. 2009-11-02 to 2010-03-04

                months = month_diff + 12;
            } else {
                // e.g. 2009-11-02 to 2010-03-04

                months = month_diff + 11;
            }
        },
        Ordering::Equal => {
            // month_diff == 0, e.g. 2009-12 to 2010-12

            if later_date >= earlier_date {
                // e.g. 2009-12-02 to 2010-12-04

                years = year_diff;
                months = 0;
            } else {
                // e.g. 2009-12-04 to 2010-12-02

                years = year_diff - 1;
                months = 11;
            }
        },
    }

    if later_date >= earlier_date {
        // e.g. 2010-01-02 to 2010-03-04, 2009-11-02 to 2010-03-04, 2009-12-02 to 2010-12-04

        if start_from_later {
            days = later_date
                .min(year_helper::get_days_in_month(earlier_year, earlier_month).unwrap())
                as i32
                - earlier_date as i32;
        } else {
            days = later_date as i32 - earlier_date as i32;
        }
    } else {
        // e.g. 2010-01-02 to 2010-03-01, 2009-11-02 to 2010-03-04, 2009-12-04 to 2010-12-02

        if start_from_later {
            if earlier_month < 12 {
                later_date = later_date
                    .min(year_helper::get_days_in_month(earlier_year, earlier_month + 1).unwrap())
            } else {
                // we don't need to handle this because the laterDate cannot be bigger than 31 (January has 31 days)
            }

            days = (later_date
                + (year_helper::get_days_in_month(earlier_year, earlier_month).unwrap()
                    - earlier_date)) as i32;
        } else {
            let days_in_month = if later_month > 1 {
                year_helper::get_days_in_month(later_year, later_month - 1).unwrap()
            } else {
                31 // year_helper::get_days_in_month(later_year - 1, 12).unwrap()
            };

            if days_in_month > earlier_date {
                days = (later_date + (days_in_month - earlier_date)) as i32;
            } else {
                days = later_date as i32;
            }
        }
    }

    _DateDiffResult {
        earlier_nanoseconds_of_day,
        later_nanoseconds_of_day,
        result: DateDiffResult {
            years,
            months,
            days,
        },
    }
}

/// Calculate the difference between two date-time values in years, months and days.
///
/// The result is positive when `to` is later than `from`, and negative when `to` is earlier than `from`.
/// If the two values are in different time zones, `to` is converted to the time zone of `from` first.
/// Then the result is calculated with the wall-clock date and time in that time zone.
/// Only complete days are counted, so a remaining time shorter than a day is dropped.
///
/// # Example
///
/// ```rust
/// # #[cfg(feature = "chrono")]
/// # {
/// use chrono::prelude::*;
/// use date_differencer::{DateDiffResult, date_diff};
///
/// let date = Local.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
/// let date_after_1_year_1_day =
///     Local.with_ymd_and_hms(2001, 1, 2, 0, 0, 0).unwrap();
///
/// assert_eq!(
///     DateDiffResult {
///         years: 1,
///         days: 1,
///         ..DateDiffResult::default()
///     },
///     date_diff(date, date_after_1_year_1_day)
/// );
/// # }
/// ```
#[inline]
pub fn date_diff<DT: DateTimeParts>(from: DT, to: DT) -> DateDiffResult {
    let to = WallClock::from_parts(&from.to_same_time_zone(to));
    let from = WallClock::from_parts(&from);

    match to.cmp(&from) {
        Ordering::Greater => _date_diff(&from, &to, false).result,
        Ordering::Less => _date_diff(&to, &from, true).result.into_neg(),
        Ordering::Equal => DateDiffResult::default(),
    }
}

/// Calculate the difference between two date-time values in years, months, days, hours, minutes, seconds and nanoseconds.
///
/// The result is positive when `to` is later than `from`, and negative when `to` is earlier than `from`.
/// If the two values are in different time zones, `to` is converted to the time zone of `from` first.
/// Then the result is calculated with the wall-clock date and time in that time zone.
///
/// # Example
///
/// ```rust
/// # #[cfg(feature = "chrono")]
/// # {
/// use chrono::prelude::*;
/// use date_differencer::{DateTimeDiffResult, date_time_diff};
///
/// let date = Local.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
/// let date_after_1_year_1_day_5_minutes =
///     Local.with_ymd_and_hms(2001, 1, 2, 0, 5, 0).unwrap();
///
/// assert_eq!(
///     DateTimeDiffResult {
///         years: 1,
///         days: 1,
///         minutes: 5,
///         ..DateTimeDiffResult::default()
///     },
///     date_time_diff(date, date_after_1_year_1_day_5_minutes)
/// );
/// # }
/// ```
#[inline]
pub fn date_time_diff<DT: DateTimeParts>(from: DT, to: DT) -> DateTimeDiffResult {
    let to = WallClock::from_parts(&from.to_same_time_zone(to));
    let from = WallClock::from_parts(&from);

    match to.cmp(&from) {
        Ordering::Greater => {
            let date_diff = _date_diff(&from, &to, false);

            let time_diff = _time_diff(
                date_diff.earlier_nanoseconds_of_day,
                date_diff.later_nanoseconds_of_day,
            );

            let date_diff = date_diff.result;

            DateTimeDiffResult {
                years:       date_diff.years,
                months:      date_diff.months,
                days:        date_diff.days,
                hours:       time_diff.hours,
                minutes:     time_diff.minutes,
                seconds:     time_diff.seconds,
                nanoseconds: time_diff.nanoseconds,
            }
        },
        Ordering::Less => {
            let date_diff = _date_diff(&to, &from, true);

            let time_diff = _time_diff(
                date_diff.earlier_nanoseconds_of_day,
                date_diff.later_nanoseconds_of_day,
            );

            let date_diff = date_diff.result;

            DateTimeDiffResult {
                years:       -date_diff.years,
                months:      -date_diff.months,
                days:        -date_diff.days,
                hours:       -time_diff.hours,
                minutes:     -time_diff.minutes,
                seconds:     -time_diff.seconds,
                nanoseconds: -time_diff.nanoseconds,
            }
        },
        Ordering::Equal => DateTimeDiffResult::default(),
    }
}
