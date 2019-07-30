use crate::time::{Holiday, Calendar, WesternCalendar};
use chrono::{NaiveDate, Datelike, Weekday};

#[derive(WesternCalendar)]
pub struct Merval;

impl Calendar for Merval {
    fn holiday(&self, date: &NaiveDate) -> Option<Holiday> {
        if date.month() == 1 && date.day() == 1 {
            Some(Holiday("New Year's Day"))
        } else if date.ordinal() == self.easter_monday(date.year()) - 4 {
            Some(Holiday("Holy Thursday"))
        } else if date.ordinal() == self.easter_monday(date.year()) - 3 {
            Some(Holiday("Good Friday"))
        } else if date.day() == 1 && date.month() == 5 {
            Some(Holiday("Labour Day"))
        } else if date.day() == 25 && date.month() == 5 {
            Some(Holiday("May Revolution"))
        } else if date.day() >= 15 && date.day() <= 21 && date.weekday() == Weekday::Mon && date.month() == 6 {
            Some(Holiday("Death of General Manuel Belgrano"))
        } else if date.day() == 9 && date.month() == 7 {
            Some(Holiday("Independence Day"))
        } else if date.day() >= 15 && date.day() <= 21 && date.weekday() == Weekday::Mon && date.month() == 8 {
            Some(Holiday("Death of General Jose de San Martin"))
        } else if (date.day() == 10 || date.day() == 11 || date.day() == 12 || date.day() == 15 || date.day() == 16) && date.weekday() == Weekday::Mon && date.month() == 10 {
            Some(Holiday("Columbus Day"))
        } else if date.day() == 8 && date.month() == 12 {
            Some(Holiday("Immaculate Conception"))
        } else if date.day() == 24 && date.month() == 12 {
            Some(Holiday("Christmas Eve"))
        } else if (date.day() == 31 || (date.day() == 30 && date.weekday() == Weekday::Fri)) && date.month() == 12 {
            Some(Holiday("New Year's Eve"))
        } else {
            None
        }
    }
}