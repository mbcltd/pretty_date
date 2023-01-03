use chrono::{Datelike, Days, NaiveDateTime};

pub enum PrettyDateRule {
    JustNow { minutes: i64 },
    Today,
    Yesterday,
    ThisWeek,
    ThisYear,
}

impl PrettyDateRule {
    pub fn does_apply_to_dates(&self, date: &NaiveDateTime, reference_date: &NaiveDateTime) -> bool {
        match self {
            PrettyDateRule::JustNow { minutes } => {
                let time_diff = reference_date.timestamp_millis() - date.timestamp_millis();
                time_diff < minutes * 1000
            },
            PrettyDateRule::Today => date.date().eq(&reference_date.date()),
            PrettyDateRule::Yesterday => {
                let yesterday = &reference_date.date().checked_sub_days(Days::new(1)).expect("Could not determine yesterday");
                date.date().eq(yesterday)
            },
            PrettyDateRule::ThisWeek =>  {
                let last_week = &reference_date.date().checked_sub_days(Days::new(7)).expect("Could not determine yesterday");
                date.date().gt(last_week)
            },
            PrettyDateRule::ThisYear => date.year() == reference_date.year(),
        }
    }

    pub fn format_date(&self, date: &NaiveDateTime) -> String {
        match self {
            PrettyDateRule::JustNow { .. } => "Just now".to_string(),
            PrettyDateRule::Today => date.format("%H:%M Today").to_string(),
            PrettyDateRule::Yesterday => date.format("%H:%M Yesterday").to_string(),
            PrettyDateRule::ThisWeek => date.format("%H:%M %A").to_string(),
            PrettyDateRule::ThisYear => date.format("%-e %B, %H:%M").to_string(),
        }
    }
}