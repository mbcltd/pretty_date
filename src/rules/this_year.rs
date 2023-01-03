use chrono::format::StrftimeItems;
use chrono::{Datelike, NaiveDateTime};
use crate::rules::PrettyDateFormatRule;


pub struct ThisYearPrettyDateFormatRule {
}

impl ThisYearPrettyDateFormatRule {
    pub fn new() -> ThisYearPrettyDateFormatRule {
        ThisYearPrettyDateFormatRule {  }
    }
}

impl PrettyDateFormatRule for ThisYearPrettyDateFormatRule {
    fn does_match(&self, date: &NaiveDateTime, reference_date: &NaiveDateTime) -> bool {
        date.year() == reference_date.year()
    }

    fn format<'a>(&self) -> StrftimeItems<'a> {
        StrftimeItems::new("%-e %B, %H:%M")
    }
}