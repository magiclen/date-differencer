use time::{OffsetDateTime, PrimitiveDateTime};

use super::DateTimeParts;

impl DateTimeParts for PrimitiveDateTime {
    #[inline]
    fn year(&self) -> i32 {
        let (year, ..) = self.to_calendar_date();

        year
    }

    #[inline]
    fn month(&self) -> u8 {
        let (_, month, _) = self.to_calendar_date();

        u8::from(month)
    }

    #[inline]
    fn day(&self) -> u8 {
        let (_, _, day) = self.to_calendar_date();

        day
    }

    #[inline]
    fn hour(&self) -> u8 {
        let (hour, ..) = self.as_hms_nano();

        hour
    }

    #[inline]
    fn minute(&self) -> u8 {
        let (_, minute, ..) = self.as_hms_nano();

        minute
    }

    #[inline]
    fn second(&self) -> u8 {
        let (_, _, second, _) = self.as_hms_nano();

        second
    }

    #[inline]
    fn nanosecond(&self) -> u32 {
        let (_, _, _, nanosecond) = self.as_hms_nano();

        nanosecond
    }
}

impl DateTimeParts for OffsetDateTime {
    #[inline]
    fn year(&self) -> i32 {
        let (year, ..) = self.to_calendar_date();

        year
    }

    #[inline]
    fn month(&self) -> u8 {
        let (_, month, _) = self.to_calendar_date();

        u8::from(month)
    }

    #[inline]
    fn day(&self) -> u8 {
        let (_, _, day) = self.to_calendar_date();

        day
    }

    #[inline]
    fn hour(&self) -> u8 {
        let (hour, ..) = self.to_hms_nano();

        hour
    }

    #[inline]
    fn minute(&self) -> u8 {
        let (_, minute, ..) = self.to_hms_nano();

        minute
    }

    #[inline]
    fn second(&self) -> u8 {
        let (_, _, second, _) = self.to_hms_nano();

        second
    }

    #[inline]
    fn nanosecond(&self) -> u32 {
        let (_, _, _, nanosecond) = self.to_hms_nano();

        nanosecond
    }
}
