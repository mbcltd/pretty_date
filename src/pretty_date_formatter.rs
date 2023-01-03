use chrono::format::StrftimeItems;
use chrono::{Local, NaiveDateTime};
use crate::pretty_date_format::PrettyDateFormat;

pub fn format_date_time(date: &NaiveDateTime, _reference_date: &NaiveDateTime) -> String {
    date.format_with_items(StrftimeItems::new("%-e %B %Y, %H:%M")).to_string()
}


pub trait PrettyDateFormatter {
    fn format_pretty(&self) -> String;
    fn format_pretty_with_reference(&self, reference_date: &NaiveDateTime) -> String;
}

impl PrettyDateFormatter for NaiveDateTime {
    fn format_pretty(&self) -> String {
        format_date_time(&self, &Local::now().naive_local())
    }

    fn format_pretty_with_reference(&self, reference_date: &NaiveDateTime) -> String {
        PrettyDateFormat::default().format_pretty_with_reference(&self, &reference_date)
    }
}



