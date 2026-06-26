use core::fmt;

use jiff::{Zoned, civil::DateTime};

use super::{AddDateTimeDiff, AddedDateTimeParts, DateTimeDiff, add_date_time_parts};

#[inline]
fn jiff_error(args: fmt::Arguments<'_>) -> jiff::Error {
    jiff::Error::from_args(args)
}

#[inline]
fn added_date_time_parts(
    from: &impl super::DateTimeParts,
    date_time_diff: &impl DateTimeDiff,
) -> Result<AddedDateTimeParts, jiff::Error> {
    add_date_time_parts(from, date_time_diff).ok_or_else(|| {
        jiff_error(format_args!("date-time addition overflowed before constructing the Jiff value"))
    })
}

#[inline]
fn jiff_year(year: i32) -> Result<i16, jiff::Error> {
    i16::try_from(year)
        .map_err(|_| jiff_error(format_args!("date-time year is outside Jiff's supported range")))
}

impl AddDateTimeDiff for DateTime {
    type Output = Result<DateTime, jiff::Error>;

    #[inline]
    fn add_date_time_diff(self, date_time_diff: &impl DateTimeDiff) -> Self::Output {
        let parts = added_date_time_parts(&self, date_time_diff)?;

        DateTime::new(
            jiff_year(parts.year)?,
            parts.month as i8,
            parts.day as i8,
            parts.hour as i8,
            parts.minute as i8,
            parts.second as i8,
            parts.nanosecond as i32,
        )
    }
}

impl AddDateTimeDiff for Zoned {
    type Output = Result<Zoned, jiff::Error>;

    #[inline]
    fn add_date_time_diff(self, date_time_diff: &impl DateTimeDiff) -> Self::Output {
        let parts = added_date_time_parts(&self, date_time_diff)?;

        self.with()
            .year(jiff_year(parts.year)?)
            .month(parts.month as i8)
            .day(parts.day as i8)
            .hour(parts.hour as i8)
            .minute(parts.minute as i8)
            .second(parts.second as i8)
            .subsec_nanosecond(parts.nanosecond as i32)
            .build()
    }
}
