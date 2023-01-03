use chrono::NaiveDateTime;
use crate::pretty_date_rule::PrettyDateRule;

pub struct PrettyDateFormat<'a> {
    pub rules: Vec<PrettyDateRule>,
    pub default_format: &'a str,
}

impl PrettyDateFormat<'_> {
    pub fn default() -> PrettyDateFormat<'static> {
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
    }

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



