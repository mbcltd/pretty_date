//! Human friendly formatting of dates.
//!
//! Provides simple, friendly, human readable formatting for the chrono NaiveDateTime,
//! for example:
//! - Within the last ten minutes: `Just now`
//! - Earlier today: `20:56 Today`
//! - Earlier in the year: `5 September, 23:56`
//!
//! Default usage:
//! ```rust
//! use chrono::NaiveDateTime;
//! use pretty_date::pretty_date_formatter::PrettyDateFormatter;
//!
//! let date = NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S").unwrap();
//! println!("{}", date.format_pretty());
//! ```
//!
//! Advanced usage:
//! ```rust
//! use chrono::NaiveDateTime;
//! use pretty_date::pretty_date_format::PrettyDateFormat;
//! use pretty_date::pretty_date_rule::PrettyDateRule;
//!
//! let date = NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S").unwrap();
//! let date_format = PrettyDateFormat {
//!             rules: vec![
//!                 PrettyDateRule::Today,
//!                 PrettyDateRule::ThisYear,
//!             ],
//!             default_format: "%-e %B %Y, %H:%M",
//!         };
//! println!("{}", date_format.format_pretty(&date));
//! ```

mod test;

pub mod pretty_date_format;
pub mod pretty_date_formatter;
pub mod pretty_date_rule;
