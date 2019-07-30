pub use chrono::NaiveDateTime;
pub use calendar::*;

mod schedule;
mod calendar;

pub mod calendars;

pub enum BusinessDayConvention {
    Following,
    ModifiedFollowing,
    Preceding,
    ModifiedPreceding,
    Unadjusted,
    HalfMonthModifiedFollowing,
    Nearest,
}

pub enum DateGenerationRule {
    Backward,
    Forward,
    Zero,
    ThirdWednesday,
    Twentieth,
    TwentiethIMM,
    OldCDS,
    CDS,
    CDS2015,
}

pub enum Frequency {
    Never,
    Once,
    Annual,
    Semiannual,
    EveryFourthMonth,
    Quarterly,
    Bimonthly,
    EveryFourthWeek,
    Biweekly,
    Weekly,
    Daily,
}

pub struct Period(pub TimeUnit, pub u32);

/// Unit of time used in conjuction with a scalar value to indicate a duration.
pub enum TimeUnit {
    Days,
    Weeks,
    Months,
    Years,
    Hours,
    Minutes,
    Seconds,
    Milliseconds,
    Microseconds,
}