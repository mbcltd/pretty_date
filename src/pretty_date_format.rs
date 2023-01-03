use chrono::format::StrftimeItems;
use chrono::{Local, NaiveDateTime};
use crate::pretty_date_formatter::PrettyDateFormatter;

pub struct PrettyDateFormat<'a> {
    // pub rules: Vec<Box<dyn PrettyDateFormatRule>>,
    pub default_format: &'a str,
}

impl<'a> PrettyDateFormat<'a> {
    pub fn default() -> PrettyDateFormat<'a> {
        PrettyDateFormat {
            default_format: "%-e %B %Y, %H:%M",
        }
    }

    pub fn format_pretty(&self, date: &NaiveDateTime) -> String {
        self.format_pretty_with_reference(date, &Local::now().naive_local())
    }

    pub fn format_pretty_with_reference(&self, date: &NaiveDateTime, reference_date: &NaiveDateTime) -> String {
        date.format_with_items(StrftimeItems::new(self.default_format)).to_string()
    }
}

pub trait PrettyDateFormatRule {
    fn matches(&self, date: &NaiveDateTime, reference_date: &NaiveDateTime) -> bool;
    fn format<'a>(&self) -> StrftimeItems<'a>;
}