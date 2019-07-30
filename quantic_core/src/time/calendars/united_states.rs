use crate::time::{Calendar, WesternCalendar, Holiday};
use chrono::{NaiveDate, Weekday, Datelike};

fn is_presidents_day(date: &NaiveDate) -> bool {
    if date.year() >= 1971 {
        (date.day() >= 15 && date.day() <= 21) && date.weekday() == Weekday::Mon && date.month() == 2
    } else {
        (date.day() == 22 || (date.day() == 23 && date.weekday() == Weekday::Mon) || (date.day() == 21 && date.weekday() == Weekday::Fri)) && date.month() == 2
    }
}

fn is_memorial_day(date: &NaiveDate) -> bool {
    if date.year() >= 1971 {
        date.day() >= 25 && date.weekday() == Weekday::Mon && date.month() == 5
    } else {
        (date.day() == 30 || (date.day() == 31 && date.weekday() == Weekday::Mon) || (date.day() == 29 && date.weekday() == Weekday::Fri)) && date.month() == 5
    }
}

fn is_labor_day(date: &NaiveDate) -> bool {
    date.day() <= 7 && date.weekday() == Weekday::Mon && date.month() == 9
}

fn is_columbus_day(date: &NaiveDate) -> bool {
    (date.day() >= 8 && date.day() <= 14) && date.weekday() == Weekday::Mon && date.month() == 10 && date.year() >= 1971
}

fn is_veterans_day(date: &NaiveDate) -> bool {
    if date.year() <= 1970 || date.year() >= 1978 {
        (date.day() == 11 || (date.day() == 12 && date.weekday() == Weekday::Mon) || (date.day() == 10 && date.weekday() == Weekday::Wed)) && date.month() == 11
    } else {
        (date.day() >= 22 && date.day() <= 28) && date.weekday() == Weekday::Mon && date.month() == 10
    }
}

#[derive(WesternCalendar)]
pub struct Settlement;

impl Calendar for Settlement {
    fn holiday(&self, date: &NaiveDate) -> Option<Holiday> {
        if ((date.day() == 1 || (date.day() == 2 && date.weekday() == Weekday::Mon)) && date.month() == 1) ||
                (date.day() == 31 && date.weekday() == Weekday::Fri && date.month() == 12) {
            
            Some(Holiday("New Year's Day"))
        } else if (date.day() >= 15 && date.day() <= 21) && date.weekday() == Weekday::Mon && date.month() == 1 && date.year() >= 1983 {
            Some(Holiday("Martin Luther King Day"))
        } else if is_presidents_day(date) {
            Some(Holiday("Presidents Day"))
        } else if is_memorial_day(date) {
            Some(Holiday("Memorial Day"))
        } else if date.day() == 4 || (date.day() == 5 && date.weekday() == Weekday::Mon) || (date.day() == 3 && date.weekday() == Weekday::Fri) && date.month() == 7 {
            Some(Holiday("Independence Day"))
        } else if is_labor_day(date) {
            Some(Holiday("Labor Day"))
        } else if is_columbus_day(date) {
            Some(Holiday("Columbus Day"))
        } else if is_veterans_day(date) {
            Some(Holiday("Veterans Day"))
        } else if (date.day() >= 22 && date.day() <= 28) && date.weekday() == Weekday::Thu && date.month() == 11 {
            Some(Holiday("Thanksgiving Day"))
        } else if (date.day() == 25 || (date.day() == 26 && date.weekday() == Weekday::Mon) || (date.day() == 24 && date.weekday() == Weekday::Fri)) && date.month() == 12 {
            Some(Holiday("Christmas Day"))
        } else {
            None
        }
    }
}

pub struct Libor;

pub struct NYSE;

pub struct Government;

pub struct NERC;

pub struct FederalReserve;
