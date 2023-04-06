use chrono::{prelude::*, LocalResult, TimeDelta};

use super::{constants::*, DateTimeDiff};

#[inline]
fn month_add(year: &mut i32, month: &mut i32, n: i32) -> Option<()> {
    *month = match month.checked_add(n) {
        Some(v) => v,
        None => return None,
    };

    if *month >= 12 {
        *year += *month / 12;
        *month %= 12;
    } else if *month < 0 {
        *year += *month / 12 - 1;

        *month = 12 - (-*month % 12);

        if *month == 12 {
            *month = 0;
        }
    }

    Some(())
}

#[inline]
fn date_add(year: &mut i32, month: &mut i32, date: &mut i32, n: i32) -> Option<()> {
    *date = match date.checked_add(n) {
        Some(v) => v,
        None => return None,
    };

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

#[inline]
fn hour_add(year: &mut i32, month: &mut i32, date: &mut i32, hour: &mut i32, n: i32) -> Option<()> {
    *hour = match hour.checked_add(n) {
        Some(v) => v,
        None => return None,
    };

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

#[inline]
fn minute_add(
    year: &mut i32,
    month: &mut i32,
    date: &mut i32,
    hour: &mut i32,
    minute: &mut i32,
    n: i32,
) -> Option<()> {
    *minute = match minute.checked_add(n) {
        Some(v) => v,
        None => return None,
    };

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
    *second = match second.checked_add(n) {
        Some(v) => v,
        None => return None,
    };

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

    *nanosecond = match nanosecond.checked_add(n) {
        Some(v) => v,
        None => return None,
    };

    if *nanosecond >= SECOND_NANOSECONDS_I32 {
        second_add(year, month, date, hour, minute, second, *nanosecond / SECOND_NANOSECONDS_I32)?;
        *nanosecond %= SECOND_NANOSECONDS_I32;
    } else if *nanosecond < 0 {
        second_add(
            year,
            month,
            date,
            hour,
            minute,
            second,
            *nanosecond / SECOND_NANOSECONDS_I32 - 1,
        )?;

        *nanosecond = SECOND_NANOSECONDS_I32 - (-*nanosecond % SECOND_NANOSECONDS_I32);

        if *nanosecond == 60 {
            *nanosecond = 0;
        }
    }

    Some(())
}

/// Calculate `from` + `date_time_diff`.
///
/// # Example
///
/// ```rust
/// use chrono::prelude::*;
/// use date_differencer::{add_date_time_diff, DateDiffResult};
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
/// ```
pub fn add_date_time_diff<Tz: TimeZone>(
    from: DateTime<Tz>,
    date_time_diff: &dyn DateTimeDiff,
) -> LocalResult<DateTime<Tz>> {
    let mut year = match from.year().checked_add(date_time_diff.years()) {
        Some(v) => v,
        None => return LocalResult::None,
    };

    let mut month = from.month0() as i32;

    if month_add(&mut year, &mut month, date_time_diff.months()).is_none() {
        return LocalResult::None;
    }

    let mut date = from.day() as i32;

    let days_in_month = year_helper::get_days_in_month(year, (month + 1) as u8).unwrap() as i32;

    if date > days_in_month {
        date = days_in_month;
    }

    if date_add(&mut year, &mut month, &mut date, date_time_diff.days()).is_none() {
        return LocalResult::None;
    }

    let mut hour = from.hour() as i32;

    if hour_add(&mut year, &mut month, &mut date, &mut hour, date_time_diff.hours()).is_none() {
        return LocalResult::None;
    }

    let mut minute = from.minute() as i32;

    if minute_add(
        &mut year,
        &mut month,
        &mut date,
        &mut hour,
        &mut minute,
        date_time_diff.minutes(),
    )
    .is_none()
    {
        return LocalResult::None;
    }

    let mut second = from.second() as i32;

    if second_add(
        &mut year,
        &mut month,
        &mut date,
        &mut hour,
        &mut minute,
        &mut second,
        date_time_diff.seconds(),
    )
    .is_none()
    {
        return LocalResult::None;
    }

    let mut nanosecond = from.nanosecond() as i32;

    if nanosecond_add(
        &mut year,
        &mut month,
        &mut date,
        &mut hour,
        &mut minute,
        &mut second,
        &mut nanosecond,
        date_time_diff.nanoseconds(),
    )
    .is_none()
    {
        return LocalResult::None;
    }

    match from.timezone().with_ymd_and_hms(
        year,
        month as u32 + 1,
        date as u32,
        hour as u32,
        minute as u32,
        second as u32,
    ) {
        LocalResult::Single(v) => {
            match v.checked_add_signed(TimeDelta::nanoseconds(nanosecond as i64)) {
                Some(v) => LocalResult::Single(v),
                None => LocalResult::None,
            }
        },
        LocalResult::Ambiguous(a, b) => {
            let delta = TimeDelta::nanoseconds(nanosecond as i64);
            LocalResult::Ambiguous(
                match a.checked_add_signed(delta) {
                    Some(v) => v,
                    None => return LocalResult::None,
                },
                match b.checked_add_signed(delta) {
                    Some(v) => v,
                    None => return LocalResult::None,
                },
            )
        },
        LocalResult::None => LocalResult::None,
    }
}
