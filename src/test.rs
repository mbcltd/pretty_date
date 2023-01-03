#[cfg(test)]
mod tests {
    use chrono::NaiveDateTime;
    use crate::pretty_date_formatter::PrettyDateFormatter;

    #[test]
    fn test_basic_date() {
        let date =
            NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
                .unwrap();

        let reference_date =
            NaiveDateTime::parse_from_str("2022-10-05 23:56:04", "%Y-%m-%d %H:%M:%S")
                .unwrap();

        assert_eq!(date.format_pretty_with_reference(&reference_date), "5 September 2015, 23:56");
    }

    #[test]
    fn test_this_year() {
        let date =
            NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
                .unwrap();

        let reference_date =
            NaiveDateTime::parse_from_str("2015-10-05 23:56:04", "%Y-%m-%d %H:%M:%S")
                .unwrap();

        assert_eq!(date.format_pretty_with_reference(&reference_date), "5 September, 23:56");
    }

    #[test]
    fn test_just_now() {
        let date =
            NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
                .unwrap();

        let reference_date =
            NaiveDateTime::parse_from_str("2015-09-05 23:56:05", "%Y-%m-%d %H:%M:%S")
                .unwrap();

        assert_eq!(date.format_pretty_with_reference(&reference_date), "Just now");
    }
}