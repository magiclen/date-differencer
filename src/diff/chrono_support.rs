use chrono::{DateTime, Datelike, NaiveDateTime, Timelike};

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
        Timelike::nanosecond(self)
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
        Timelike::nanosecond(self)
    }
}
