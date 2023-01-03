use chrono::format::StrftimeItems;
use chrono::{Local, NaiveDateTime};
use crate::pretty_date_formatter::PrettyDateFormatter;

pub struct PrettyDateFormat<'a> {
    pub rules: Vec<Box<dyn PrettyDateFormatRule>>,
    pub default_format: &'a str,
}

impl<'a> PrettyDateFormat<'a> {
    pub fn default() -> PrettyDateFormat<'a> {
        PrettyDateFormat {
            rules: vec![Box::new(JustNowPrettyDateFormatRule::new())],
            default_format: "%-e %B %Y, %H:%M",
        }
    }

    pub fn format_pretty(&self, date: &NaiveDateTime) -> String {
        self.format_pretty_with_reference(date, &Local::now().naive_local())
    }

    pub fn format_pretty_with_reference(&self, date: &NaiveDateTime, reference_date: &NaiveDateTime) -> String {
        let matching_rule =
            self
                .rules
                .iter()
                .find(|&the_rule| the_rule.does_match(date, reference_date));

        match matching_rule {
            Some(rule) => date.format_with_items(rule.format()).to_string(),
            None => date.format_with_items(StrftimeItems::new(self.default_format)).to_string()
        }
    }
}

pub trait PrettyDateFormatRule {
    fn does_match(&self, date: &NaiveDateTime, reference_date: &NaiveDateTime) -> bool;
    fn format<'a>(&self) -> StrftimeItems<'a>;
}

struct JustNowPrettyDateFormatRule {}

impl JustNowPrettyDateFormatRule {
    pub fn new() -> JustNowPrettyDateFormatRule {
        JustNowPrettyDateFormatRule {}
    }
}

impl PrettyDateFormatRule for JustNowPrettyDateFormatRule {
    fn does_match(&self, date: &NaiveDateTime, reference_date: &NaiveDateTime) -> bool {
        let time_diff = reference_date.timestamp_millis() - date.timestamp_millis();
        time_diff < 60000
    }

    fn format<'a>(&self) -> StrftimeItems<'a> {
        StrftimeItems::new("Just now")
    }
}