use chrono::format::StrftimeItems;
use chrono::NaiveDateTime;
use crate::rules::PrettyDateFormatRule;

pub struct TodayPrettyDateFormatRule {
}

impl TodayPrettyDateFormatRule {
    pub fn new() -> TodayPrettyDateFormatRule {
        TodayPrettyDateFormatRule { }
    }
}

impl PrettyDateFormatRule for TodayPrettyDateFormatRule {
    fn does_match(&self, date: &NaiveDateTime, reference_date: &NaiveDateTime) -> bool {
        date.date().eq(&reference_date.date())
    }

    fn format<'a>(&self) -> StrftimeItems<'a> {
        StrftimeItems::new("%H:%M Today")
    }
}