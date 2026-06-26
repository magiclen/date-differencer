#[cfg(feature = "chrono")]
mod chrono_support;
#[cfg(feature = "time")]
mod time_support;

#[cfg(any(feature = "chrono", feature = "time"))]
use super::constants::*;
use super::{DateTimeDiff, DateTimeParts};

#[cfg(any(feature = "chrono", feature = "time"))]
#[derive(Debug, Clone, Copy)]
struct AddedDateTimeParts {
    year:       i32,
    month:      u8,
    day:        u8,
    hour:       u8,
    minute:     u8,
    second:     u8,
    nanosecond: u32,
}

#[cfg(any(feature = "chrono", feature = "time"))]
#[inline]
fn month_add(year: &mut i32, month: &mut i32, n: i32) -> Option<()> {
    *month = month.checked_add(n)?;

    if *month >= 12 {
        *year = year.checked_add(*month / 12)?;
        *month %= 12;
    } else if *month < 0 {
        *year = year.checked_add(*month / 12 - 1)?;

        *month = 12 - (-*month % 12);

        if *month == 12 {
            *month = 0;
        }
    }

    Some(())
}

#[cfg(any(feature = "chrono", feature = "time"))]
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

#[cfg(any(feature = "chrono", feature = "time"))]
#[inline]
fn hour_add(year: &mut i32, month: &mut i32, date: &mut i32, hour: &mut i32, n: i32) -> Option<()> {
    *hour = hour.checked_add(n)?;

    if *hour >= 24 {
        date_add(year, month, date, *hour / 24)?;
        *hour %= 24;
    } else if *hour < 0 {
        date_add(year, month, date, *hour / 24 - 1)?;

        *hour = 24 - (-*hour % 24);

        if *hour == 24 {
            *hour = 0;
        }
    }

    Some(())
}

#[cfg(any(feature = "chrono", feature = "time"))]
#[inline]
fn minute_add(
    year: &mut i32,
    month: &mut i32,
    date: &mut i32,
    hour: &mut i32,
    minute: &mut i32,
    n: i32,
) -> Option<()> {
    *minute = minute.checked_add(n)?;

    if *minute >= 60 {
        hour_add(year, month, date, hour, *minute / 60)?;
        *minute %= 60;
    } else if *minute < 0 {
        hour_add(year, month, date, hour, *minute / 60 - 1)?;

        *minute = 60 - (-*minute % 60);

        if *minute == 60 {
            *minute = 0;
        }
    }

    Some(())
}

#[cfg(any(feature = "chrono", feature = "time"))]
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
    *second = second.checked_add(n)?;

    if *second >= 60 {
        minute_add(year, month, date, hour, minute, *second / 60)?;
        *second %= 60;
    } else if *second < 0 {
        minute_add(year, month, date, hour, minute, *second / 60 - 1)?;

        *second = 60 - (-*second % 60);

        if *second == 60 {
            *second = 0;
        }
    }

    Some(())
}

#[cfg(any(feature = "chrono", feature = "time"))]
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
    const SECOND_NANOSECONDS_I32: i32 = SECOND_NANOSECONDS as i32;

    let total_nanoseconds = nanosecond.checked_add(n)?;
    let seconds = total_nanoseconds.div_euclid(SECOND_NANOSECONDS_I32);
    let normalized_nanoseconds = total_nanoseconds.rem_euclid(SECOND_NANOSECONDS_I32);

    if seconds != 0 {
        second_add(year, month, date, hour, minute, second, seconds)?;
    }

    *nanosecond = normalized_nanoseconds;

    Some(())
}

#[cfg(any(feature = "chrono", feature = "time"))]
fn add_date_time_parts(
    from: &impl DateTimeParts,
    date_time_diff: &impl DateTimeDiff,
) -> Option<AddedDateTimeParts> {
    let mut year = from.year().checked_add(date_time_diff.years())?;
    let mut month = from.month() as i32 - 1;

    month_add(&mut year, &mut month, date_time_diff.months())?;

    let mut date = from.day() as i32;

    let days_in_month = year_helper::get_days_in_month(year, (month + 1) as u8).unwrap() as i32;

    if date > days_in_month {
        date = days_in_month;
    }

    date_add(&mut year, &mut month, &mut date, date_time_diff.days())?;

    let mut hour = from.hour() as i32;

    hour_add(&mut year, &mut month, &mut date, &mut hour, date_time_diff.hours())?;

    let mut minute = from.minute() as i32;

    minute_add(&mut year, &mut month, &mut date, &mut hour, &mut minute, date_time_diff.minutes())?;

    let mut second = from.second() as i32;

    second_add(
        &mut year,
        &mut month,
        &mut date,
        &mut hour,
        &mut minute,
        &mut second,
        date_time_diff.seconds(),
    )?;

    let mut nanosecond = from.nanosecond() as i32;

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

    Some(AddedDateTimeParts {
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
pub trait AddDateTimeDiff: DateTimeParts {
    type Output;

    fn add_date_time_diff(self, date_time_diff: &impl DateTimeDiff) -> Self::Output;
}

/// Calculate `from` + `date_time_diff`.
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
