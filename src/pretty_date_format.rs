use chrono::format::StrftimeItems;
use chrono::{Local, NaiveDateTime};
use crate::rules::today::TodayPrettyDateFormatRule;
use crate::rules::just_now::JustNowPrettyDateFormatRule;
use crate::rules::PrettyDateFormatRule;
use crate::rules::this_year::ThisYearPrettyDateFormatRule;
use crate::rules::yesterday::YesterdayPrettyDateFormatRule;

pub struct PrettyDateFormat<'a> {
    pub rules: Vec<Box<dyn PrettyDateFormatRule>>,
    pub default_format: &'a str,
}

impl<'a> PrettyDateFormat<'a> {
    pub fn default() -> PrettyDateFormat<'a> {
        PrettyDateFormat {
            rules: vec![
                Box::new(JustNowPrettyDateFormatRule::new(60)),
                Box::new(TodayPrettyDateFormatRule::new()),
                Box::new(YesterdayPrettyDateFormatRule::new()),
                Box::new(ThisYearPrettyDateFormatRule::new()),
            ],
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



