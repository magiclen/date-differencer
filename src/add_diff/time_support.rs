use time::{Date, Month, OffsetDateTime, PrimitiveDateTime, Time, UtcDateTime};

use super::{AddDateTimeDiff, AddedDateTimeParts, DateTimeDiff, add_date_time_parts};

#[inline]
fn date_time_from_parts(parts: AddedDateTimeParts) -> Option<(Date, Time)> {
    let date =
        Date::from_calendar_date(parts.year, Month::try_from(parts.month).ok()?, parts.day).ok()?;

    let time =
        Time::from_hms_nano(parts.hour, parts.minute, parts.second, parts.nanosecond).ok()?;

    Some((date, time))
}

impl AddDateTimeDiff for PrimitiveDateTime {
    type Output = Option<PrimitiveDateTime>;

    #[inline]
    fn add_date_time_diff(self, date_time_diff: &impl DateTimeDiff) -> Self::Output {
        let (date, time) = date_time_from_parts(add_date_time_parts(&self, date_time_diff)?)?;

        Some(PrimitiveDateTime::new(date, time))
    }
}

impl AddDateTimeDiff for OffsetDateTime {
    type Output = Option<OffsetDateTime>;

    #[inline]
    fn add_date_time_diff(self, date_time_diff: &impl DateTimeDiff) -> Self::Output {
        let (date, time) = date_time_from_parts(add_date_time_parts(&self, date_time_diff)?)?;

        Some(OffsetDateTime::new_in_offset(date, time, self.offset()))
    }
}

impl AddDateTimeDiff for UtcDateTime {
    type Output = Option<UtcDateTime>;

    #[inline]
    fn add_date_time_diff(self, date_time_diff: &impl DateTimeDiff) -> Self::Output {
        let (date, time) = date_time_from_parts(add_date_time_parts(&self, date_time_diff)?)?;

        Some(UtcDateTime::new(date, time))
    }
}
