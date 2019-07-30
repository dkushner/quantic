use chrono::{NaiveDate};
use crate::time::{Holiday, Calendar, WesternCalendar};

#[derive(WesternCalendar)]
pub struct NullCalendar;

impl Calendar for NullCalendar {
    fn holiday(&self, date: &NaiveDate) -> Option<Holiday> {
        None
    }
}

