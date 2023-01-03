use chrono::format::StrftimeItems;
use chrono::NaiveDateTime;
use crate::rules::PrettyDateFormatRule;

pub struct JustNowPrettyDateFormatRule {
    minutes: i64,
}

impl JustNowPrettyDateFormatRule {
    pub fn new(minutes: i64) -> JustNowPrettyDateFormatRule {
        JustNowPrettyDateFormatRule { minutes }
    }
}

impl PrettyDateFormatRule for JustNowPrettyDateFormatRule {
    fn does_match(&self, date: &NaiveDateTime, reference_date: &NaiveDateTime) -> bool {
        let time_diff = reference_date.timestamp_millis() - date.timestamp_millis();
        time_diff < self.minutes * 1000
    }

    fn format<'a>(&self) -> StrftimeItems<'a> {
        StrftimeItems::new("Just now")
    }
}