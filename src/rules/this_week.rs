use chrono::format::StrftimeItems;
use chrono::{Days, NaiveDateTime};
use crate::rules::PrettyDateFormatRule;

pub struct ThisWeekPrettyDateFormatRule {
}

impl ThisWeekPrettyDateFormatRule {
    pub fn new() -> ThisWeekPrettyDateFormatRule {
        ThisWeekPrettyDateFormatRule { }
    }
}

impl PrettyDateFormatRule for ThisWeekPrettyDateFormatRule {
    fn does_match(&self, date: &NaiveDateTime, reference_date: &NaiveDateTime) -> bool {
        let yesterday = &reference_date.date().checked_sub_days(Days::new(7)).expect("Could not determine yesterday");
        date.date().gt(yesterday)
    }

    fn format<'a>(&self) -> StrftimeItems<'a> {
        StrftimeItems::new("%H:%M %A")
    }
}