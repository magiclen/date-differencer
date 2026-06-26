use chrono::{DateTime, Duration, LocalResult, NaiveDate, NaiveDateTime, TimeZone};

use super::{AddDateTimeDiff, AddedDateTimeParts, DateTimeDiff, add_date_time_parts};

#[inline]
fn naive_date_time_from_parts(parts: AddedDateTimeParts) -> Option<NaiveDateTime> {
    NaiveDate::from_ymd_opt(parts.year, parts.month as u32, parts.day as u32)?.and_hms_nano_opt(
        parts.hour as u32,
        parts.minute as u32,
        parts.second as u32,
        parts.nanosecond,
    )
}

impl AddDateTimeDiff for NaiveDateTime {
    type Output = Option<NaiveDateTime>;

    #[inline]
    fn add_date_time_diff(self, date_time_diff: &impl DateTimeDiff) -> Self::Output {
        let parts = add_date_time_parts(&self, date_time_diff)?;

        naive_date_time_from_parts(parts)
    }
}

impl<Tz> AddDateTimeDiff for DateTime<Tz>
where
    Tz: TimeZone,
    DateTime<Tz>: Ord,
{
    type Output = LocalResult<DateTime<Tz>>;

    #[inline]
    fn add_date_time_diff(self, date_time_diff: &impl DateTimeDiff) -> Self::Output {
        let parts = match add_date_time_parts(&self, date_time_diff) {
            Some(parts) => parts,
            None => return LocalResult::None,
        };

        match self.timezone().with_ymd_and_hms(
            parts.year,
            parts.month as u32,
            parts.day as u32,
            parts.hour as u32,
            parts.minute as u32,
            parts.second as u32,
        ) {
            LocalResult::Single(v) => {
                match v.checked_add_signed(Duration::nanoseconds(parts.nanosecond as i64)) {
                    Some(v) => LocalResult::Single(v),
                    None => LocalResult::None,
                }
            },
            LocalResult::Ambiguous(a, b) => {
                let delta = Duration::nanoseconds(parts.nanosecond as i64);
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
}
