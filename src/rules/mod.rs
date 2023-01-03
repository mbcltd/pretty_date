use chrono::format::StrftimeItems;
use chrono::NaiveDateTime;

pub mod just_now;
pub mod this_year;
pub mod today;
pub mod yesterday;
pub mod this_week;

pub trait PrettyDateFormatRule {
    fn does_match(&self, date: &NaiveDateTime, reference_date: &NaiveDateTime) -> bool;
    fn format<'a>(&self) -> StrftimeItems<'a>;
}