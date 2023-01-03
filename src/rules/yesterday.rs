use chrono::format::StrftimeItems;
use chrono::{Days, NaiveDateTime};
use crate::rules::PrettyDateFormatRule;

pub struct YesterdayPrettyDateFormatRule {
}

impl YesterdayPrettyDateFormatRule {
    pub fn new() -> YesterdayPrettyDateFormatRule {
        YesterdayPrettyDateFormatRule { }
    }
}

impl PrettyDateFormatRule for YesterdayPrettyDateFormatRule {
    fn does_match(&self, date: &NaiveDateTime, reference_date: &NaiveDateTime) -> bool {
        let yesterday = &reference_date.date().checked_sub_days(Days::new(1)).expect("Could not determine yesterday");
        date.date().eq(yesterday)
    }

    fn format<'a>(&self) -> StrftimeItems<'a> {
        StrftimeItems::new("%H:%M Yesterday")
    }
}