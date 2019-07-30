use chrono::{NaiveDate, Duration, Utc};
use crate::time::{Calendar, BusinessDayConvention, DateGenerationRule, Frequency, Period, TimeUnit};
use crate::time::calendars::NullCalendar;

pub struct Schedule<'a> {
    effective_date: NaiveDate,
    termination_date: NaiveDate,
    convention: BusinessDayConvention,
    end_convention: BusinessDayConvention,
    calendar: &'a dyn Calendar,
    frequency: Frequency,
}

impl<'a> Schedule<'a> { }

struct ScheduleBuilder<'a> { 
    effective_date: NaiveDate,
    termination_date: NaiveDate,
    convention: BusinessDayConvention,
    end_convention: BusinessDayConvention,
    generation_rule: DateGenerationRule,
    frequency: Frequency,
    tenor: Period,
    calendar: &'a dyn Calendar,
}

impl<'a> ScheduleBuilder<'a> {
    fn new() -> Self {
        ScheduleBuilder { ..Default::default() }
    }

    fn from(mut self, date: NaiveDate) -> Self {
        self.effective_date = date;
        self
    }

    fn to(mut self, date: NaiveDate) -> Self {
        self.termination_date = date;
        self
    }

    fn with_convention(mut self, convention: BusinessDayConvention) -> Self {
        self.convention = convention;
        self
    }

    fn with_end_convention(mut self, convention: BusinessDayConvention) -> Self {
        self.end_convention = convention;
        self
    }

    fn forward(mut self) -> Self {
        self.generation_rule = DateGenerationRule::Forward;
        self
    }

    fn backward(mut self) -> Self {
        self.generation_rule = DateGenerationRule::Backward;
        self
    }

    fn with_calendar(mut self, calendar: &'a dyn Calendar) -> Self {
        self.calendar = calendar;
        self
    }

    fn with_frequency(mut self, frequency: Frequency) -> Self {
        self.frequency = frequency;
        self
    }

    fn with_tenor(mut self, tenor: Period) -> Self {
        self.tenor = tenor;
        self
    }

    fn build(self) -> Schedule<'a> {
        Schedule {
            effective_date: self.effective_date,
            termination_date: self.termination_date,
            calendar: self.calendar,
            convention: self.convention,
            end_convention: self.end_convention,
            frequency: self.frequency,
        }
    }
}

impl<'a> Default for ScheduleBuilder<'a> {
    fn default() -> Self {
        ScheduleBuilder {
            effective_date: Utc::now().naive_utc().date(),
            termination_date: Utc::now().naive_utc().date(),
            convention: BusinessDayConvention::Unadjusted,
            end_convention: BusinessDayConvention::Unadjusted,
            generation_rule: DateGenerationRule::Forward,
            calendar: &NullCalendar,
            frequency: Frequency::Daily,
            tenor: Period(TimeUnit::Days, 1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_daily_schedule() {
        let start_date = NaiveDate::from_ymd(2019, 1, 1);
        let end_date = start_date + Duration::weeks(2);
        let calendar = crate::time::calendars::united_states::Settlement;

        let schedule = ScheduleBuilder::new()
            .from(start_date)
            .to(end_date)
            .with_calendar(&calendar)
            .with_convention(BusinessDayConvention::Preceding)
            .build();
    }
}