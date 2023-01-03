use chrono::{Local, NaiveDateTime};
use crate::pretty_date_rule::PrettyDateRule;

/// Definition of a pretty date format
pub struct PrettyDateFormat<'a> {
    /// The rules to be applied in order of precedence
    pub rules: Vec<PrettyDateRule>,

    /// The default format string to be used if no rules apply
    pub default_format: &'a str,
}

impl PrettyDateFormat<'_> {
    /// Format a date to a pretty human readable String with reference to the current local system time
    pub fn format_pretty(&self, date: &NaiveDateTime) -> String {
        self.format_pretty_with_reference(date, &Local::now().naive_local())
    }

    /// Format a date to a pretty human readable String with reference to a specific date
    pub fn format_pretty_with_reference(&self, date: &NaiveDateTime, reference_date: &NaiveDateTime) -> String {
        let matching_rule =
            self
                .rules
                .iter()
                .find(|the_rule| the_rule.does_apply_to_dates(date, reference_date));

        match matching_rule {
            Some(rule) => rule.format_date(date),
            None => date.format(self.default_format).to_string()
        }
    }
}





