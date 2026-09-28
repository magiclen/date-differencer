#[cfg(feature = "chrono")]
mod chrono_support;
#[cfg(feature = "jiff")]
mod jiff_support;
#[cfg(feature = "time")]
mod time_support;

#[cfg(any(feature = "chrono", feature = "jiff", feature = "time"))]
use super::constants::*;
#[cfg(any(feature = "chrono", feature = "jiff", feature = "time"))]
use super::diff::DateTimeFields;
use super::{DateTimeDiff, DateTimeParts};

#[cfg(any(feature = "chrono", feature = "jiff", feature = "time"))]
#[inline]
fn month_add(year: &mut i32, month: &mut i32, n: i32) -> Option<()> {
    // The carry functions add in `i64` so a large `n` cannot overflow, and use Euclidean division so a negative total borrows the right amount.
    let total = i64::from(*month) + i64::from(n);

    if (0..12).contains(&total) {
        *month = total as i32;
    } else {
        *year = year.checked_add(total.div_euclid(12) as i32)?;
        *month = total.rem_euclid(12) as i32;
    }

    Some(())
}

#[cfg(any(feature = "chrono", feature = "jiff", feature = "time"))]
#[inline]
fn date_add(year: &mut i32, month: &mut i32, date: &mut i32, n: i32) -> Option<()> {
    *date = date.checked_add(n)?;

    if *date == 0 {
        month_add(year, month, -1)?;

        *date = year_helper::get_days_in_month(*year, (*month + 1) as u8).unwrap() as i32;
    } else if *date > 28 {
        loop {
            let days_in_month =
                year_helper::get_days_in_month(*year, (*month + 1) as u8).unwrap() as i32;

            if *date <= days_in_month {
                break;
            }

            month_add(year, month, 1)?;

            *date -= days_in_month;
        }
    } else if *date < 0 {
        loop {
            month_add(year, month, -1)?;

            let days_in_month =
                year_helper::get_days_in_month(*year, (*month + 1) as u8).unwrap() as i32;

            if -*date < days_in_month {
                *date += days_in_month;
                break;
            }

            *date += days_in_month;
        }
    }

    Some(())
}

#[cfg(any(feature = "chrono", feature = "jiff", feature = "time"))]
#[inline]
fn hour_add(year: &mut i32, month: &mut i32, date: &mut i32, hour: &mut i32, n: i32) -> Option<()> {
    let total = i64::from(*hour) + i64::from(n);

    if (0..24).contains(&total) {
        *hour = total as i32;
    } else {
        date_add(year, month, date, total.div_euclid(24) as i32)?;
        *hour = total.rem_euclid(24) as i32;
    }

    Some(())
}

#[cfg(any(feature = "chrono", feature = "jiff", feature = "time"))]
#[inline]
fn minute_add(
    year: &mut i32,
    month: &mut i32,
    date: &mut i32,
    hour: &mut i32,
    minute: &mut i32,
    n: i32,
) -> Option<()> {
    let total = i64::from(*minute) + i64::from(n);

    if (0..60).contains(&total) {
        *minute = total as i32;
    } else {
        hour_add(year, month, date, hour, total.div_euclid(60) as i32)?;
        *minute = total.rem_euclid(60) as i32;
    }

    Some(())
}

#[cfg(any(feature = "chrono", feature = "jiff", feature = "time"))]
#[inline]
fn second_add(
    year: &mut i32,
    month: &mut i32,
    date: &mut i32,
    hour: &mut i32,
    minute: &mut i32,
    second: &mut i32,
    n: i32,
) -> Option<()> {
    let total = i64::from(*second) + i64::from(n);

    if (0..60).contains(&total) {
        *second = total as i32;
    } else {
        minute_add(year, month, date, hour, minute, total.div_euclid(60) as i32)?;
        *second = total.rem_euclid(60) as i32;
    }

    Some(())
}

#[cfg(any(feature = "chrono", feature = "jiff", feature = "time"))]
#[allow(clippy::too_many_arguments)]
#[inline]
fn nanosecond_add(
    year: &mut i32,
    month: &mut i32,
    date: &mut i32,
    hour: &mut i32,
    minute: &mut i32,
    second: &mut i32,
    nanosecond: &mut i32,
    n: i32,
) -> Option<()> {
    const SECOND_NANOSECONDS_I64: i64 = SECOND_NANOSECONDS as i64;

    let total = i64::from(*nanosecond) + i64::from(n);

    if (0..SECOND_NANOSECONDS_I64).contains(&total) {
        *nanosecond = total as i32;
    } else {
        second_add(
            year,
            month,
            date,
            hour,
            minute,
            second,
            total.div_euclid(SECOND_NANOSECONDS_I64) as i32,
        )?;
        *nanosecond = total.rem_euclid(SECOND_NANOSECONDS_I64) as i32;
    }

    Some(())
}

#[cfg(any(feature = "chrono", feature = "jiff", feature = "time"))]
fn add_date_time_parts(
    from: &impl DateTimeParts,
    date_time_diff: &impl DateTimeDiff,
) -> Option<DateTimeFields> {
    let from = DateTimeFields::from_parts(from);

    let mut year = from.year.checked_add(date_time_diff.years())?;
    let mut month = from.month as i32 - 1;

    month_add(&mut year, &mut month, date_time_diff.months())?;

    let mut date = from.day as i32;

    let days_in_month = year_helper::get_days_in_month(year, (month + 1) as u8).unwrap() as i32;

    if date > days_in_month {
        date = days_in_month;
    }

    date_add(&mut year, &mut month, &mut date, date_time_diff.days())?;

    let mut hour = from.hour as i32;

    hour_add(&mut year, &mut month, &mut date, &mut hour, date_time_diff.hours())?;

    let mut minute = from.minute as i32;

    minute_add(&mut year, &mut month, &mut date, &mut hour, &mut minute, date_time_diff.minutes())?;

    let mut second = from.second as i32;

    second_add(
        &mut year,
        &mut month,
        &mut date,
        &mut hour,
        &mut minute,
        &mut second,
        date_time_diff.seconds(),
    )?;

    let mut nanosecond = from.nanosecond as i32;

    nanosecond_add(
        &mut year,
        &mut month,
        &mut date,
        &mut hour,
        &mut minute,
        &mut second,
        &mut nanosecond,
        date_time_diff.nanoseconds(),
    )?;

    Some(DateTimeFields {
        year,
        month: (month + 1) as u8,
        day: date as u8,
        hour: hour as u8,
        minute: minute as u8,
        second: second as u8,
        nanosecond: nanosecond as u32,
    })
}

/// A trait for date-time types that can apply a `DateTimeDiff`.
///
/// The `Output` type depends on the date-time type.
///
/// * `chrono::NaiveDateTime`: `Option<NaiveDateTime>`, which is `None` if the result is out of range.
/// * `chrono::DateTime<Tz>`: `LocalResult<DateTime<Tz>>`, which is `None` if the result does not exist in the time zone (for example, in a DST gap) and `Ambiguous` if it exists twice (for example, in a DST overlap).
/// * `time::PrimitiveDateTime`, `time::OffsetDateTime` and `time::UtcDateTime`: `Option<_>`, which is `None` if the result is out of range. `OffsetDateTime` keeps the offset of `from`.
/// * `jiff::civil::DateTime` and `jiff::Zoned`: `Result<_, jiff::Error>`. `Zoned` resolves a DST gap or overlap with the default rules of Jiff.
pub trait AddDateTimeDiff: DateTimeParts {
    type Output;

    fn add_date_time_diff(self, date_time_diff: &impl DateTimeDiff) -> Self::Output;
}

/// Calculate `from` + `date_time_diff`.
///
/// Years and months are added first, and the day is changed to the last day of the month if that month is shorter.
/// Then days, hours, minutes, seconds and nanoseconds are added in this order.
/// See `AddDateTimeDiff` for the output type.
///
/// # Example
///
/// ```rust
/// # #[cfg(feature = "chrono")]
/// # {
/// use chrono::prelude::*;
/// use date_differencer::{DateDiffResult, add_date_time_diff};
///
/// let date = Local.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
///
/// let date_after_1_year_1_day = add_date_time_diff(date, &DateDiffResult {
///     years: 1,
///     days: 1,
///     ..DateDiffResult::default()
/// })
/// .unwrap();
///
/// assert_eq!(
///     Local.with_ymd_and_hms(2001, 1, 2, 0, 0, 0).unwrap(),
///     date_after_1_year_1_day
/// )
/// # }
/// ```
#[inline]
pub fn add_date_time_diff<DT: AddDateTimeDiff>(
    from: DT,
    date_time_diff: &impl DateTimeDiff,
) -> DT::Output {
    from.add_date_time_diff(date_time_diff)
}
