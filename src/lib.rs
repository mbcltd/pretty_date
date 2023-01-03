use chrono::{NaiveDateTime};
use chrono::prelude::*;

mod test;

pub fn format_date_time(date: &NaiveDateTime, _reference_date: &NaiveDateTime) -> String {
    date.format("%e %B %Y, %H:%M").to_string().trim().to_string()
}

pub trait PrettyDateFormat {
    fn format_pretty(&self) -> String;
    fn format_pretty_with_reference(&self, reference_date: &NaiveDateTime) -> String;
}

impl PrettyDateFormat for NaiveDateTime {
    fn format_pretty(&self) -> String {
        format_date_time(&self, &Local::now().naive_local())
    }

    fn format_pretty_with_reference(&self, reference_date: &NaiveDateTime) -> String {
        format_date_time(&self, reference_date)
    }
}