use chrono::{Local, NaiveDateTime};
use lazy_static::lazy_static;
use crate::pretty_date_format::PrettyDateFormat;
use crate::pretty_date_rule::PrettyDateRule;

pub trait PrettyDateFormatter {
    fn format_pretty(&self) -> String;
    fn format_pretty_with_reference(&self, reference_date: &NaiveDateTime) -> String;
}

lazy_static! {
    static ref DEFAULT_FORMAT: PrettyDateFormat<'static> = {
        PrettyDateFormat {
            rules: vec![
                PrettyDateRule::JustNow { minutes: 60 },
                PrettyDateRule::Today,
                PrettyDateRule::Yesterday,
                PrettyDateRule::ThisWeek,
                PrettyDateRule::ThisYear,
            ],
            default_format: "%-e %B %Y, %H:%M",
        }
    };
}

impl PrettyDateFormatter for NaiveDateTime {
    fn format_pretty(&self) -> String {
        self.format_pretty_with_reference(&Local::now().naive_local())
    }

    fn format_pretty_with_reference(&self, reference_date: &NaiveDateTime) -> String {
        DEFAULT_FORMAT.format_pretty_with_reference(&self, &reference_date)
    }
}



