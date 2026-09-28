use chrono::{DateTime, Datelike, NaiveDateTime, Offset, Timelike};

use super::DateTimeParts;

impl<Tz> DateTimeParts for DateTime<Tz>
where
    Tz: chrono::TimeZone,
    DateTime<Tz>: Ord,
{
    #[inline]
    fn year(&self) -> i32 {
        Datelike::year(self)
    }

    #[inline]
    fn month(&self) -> u8 {
        Datelike::month(self) as u8
    }

    #[inline]
    fn day(&self) -> u8 {
        Datelike::day(self) as u8
    }

    #[inline]
    fn hour(&self) -> u8 {
        Timelike::hour(self) as u8
    }

    #[inline]
    fn minute(&self) -> u8 {
        Timelike::minute(self) as u8
    }

    #[inline]
    fn second(&self) -> u8 {
        Timelike::second(self) as u8
    }

    #[inline]
    fn nanosecond(&self) -> u32 {
        // chrono represents a leap second with 1_000_000_000 or more nanoseconds, so treat it as the last nanosecond of the second.
        Timelike::nanosecond(self).min(999_999_999)
    }

    #[inline]
    fn all_parts(&self) -> (i32, u8, u8, u8, u8, u8, u32) {
        // Calculate the local date-time only once instead of once per field.
        self.naive_local().all_parts()
    }

    #[inline]
    fn to_same_time_zone(&self, other: Self) -> Self {
        // Looking up a time zone such as `Local` is slow, so skip the conversion when the offsets are already the same.
        if self.offset().fix() == other.offset().fix() {
            other
        } else {
            other.with_timezone(&self.timezone())
        }
    }
}

impl DateTimeParts for NaiveDateTime {
    #[inline]
    fn year(&self) -> i32 {
        Datelike::year(self)
    }

    #[inline]
    fn month(&self) -> u8 {
        Datelike::month(self) as u8
    }

    #[inline]
    fn day(&self) -> u8 {
        Datelike::day(self) as u8
    }

    #[inline]
    fn hour(&self) -> u8 {
        Timelike::hour(self) as u8
    }

    #[inline]
    fn minute(&self) -> u8 {
        Timelike::minute(self) as u8
    }

    #[inline]
    fn second(&self) -> u8 {
        Timelike::second(self) as u8
    }

    #[inline]
    fn nanosecond(&self) -> u32 {
        // chrono represents a leap second with 1_000_000_000 or more nanoseconds, so treat it as the last nanosecond of the second.
        Timelike::nanosecond(self).min(999_999_999)
    }
}
