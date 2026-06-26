use jiff::{Zoned, civil::DateTime};

use super::DateTimeParts;

impl DateTimeParts for DateTime {
    #[inline]
    fn year(&self) -> i32 {
        i32::from(DateTime::year(*self))
    }

    #[inline]
    fn month(&self) -> u8 {
        DateTime::month(*self) as u8
    }

    #[inline]
    fn day(&self) -> u8 {
        DateTime::day(*self) as u8
    }

    #[inline]
    fn hour(&self) -> u8 {
        DateTime::hour(*self) as u8
    }

    #[inline]
    fn minute(&self) -> u8 {
        DateTime::minute(*self) as u8
    }

    #[inline]
    fn second(&self) -> u8 {
        DateTime::second(*self) as u8
    }

    #[inline]
    fn nanosecond(&self) -> u32 {
        DateTime::subsec_nanosecond(*self) as u32
    }
}

impl DateTimeParts for Zoned {
    #[inline]
    fn year(&self) -> i32 {
        i32::from(self.year())
    }

    #[inline]
    fn month(&self) -> u8 {
        self.month() as u8
    }

    #[inline]
    fn day(&self) -> u8 {
        self.day() as u8
    }

    #[inline]
    fn hour(&self) -> u8 {
        self.hour() as u8
    }

    #[inline]
    fn minute(&self) -> u8 {
        self.minute() as u8
    }

    #[inline]
    fn second(&self) -> u8 {
        self.second() as u8
    }

    #[inline]
    fn nanosecond(&self) -> u32 {
        self.subsec_nanosecond() as u32
    }
}
