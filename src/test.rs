#[cfg(test)]
mod tests {
    use chrono::NaiveDateTime;
    use crate::pretty_date_formatter::PrettyDateFormatter;

    #[test]
    fn test_basic_date_1() {
        let date =
            NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")
                .unwrap();

        let reference_date =
            NaiveDateTime::parse_from_str("2022-10-05 23:56:04", "%Y-%m-%d %H:%M:%S")
                .unwrap();

        assert_eq!(date.format_pretty_with_reference(&reference_date), "5 September 2015, 23:56");
    }

    #[test]
    fn test_basic_date_2() {
        let date =
            NaiveDateTime::parse_from_str("2015-08-30 20:56:04", "%Y-%m-%d %H:%M:%S")
                .unwrap();

        let reference_date =
            NaiveDateTime::parse_from_str("2016-09-06 23:56:05", "%Y-%m-%d %H:%M:%S")
                .unwrap();

        assert_eq!(date.format_pretty_with_reference(&reference_date), "30 August 2015, 20:56");
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
    fn test_today() {
        let date =
            NaiveDateTime::parse_from_str("2015-09-05 20:56:04", "%Y-%m-%d %H:%M:%S")
                .unwrap();

        let reference_date =
            NaiveDateTime::parse_from_str("2015-09-05 23:56:05", "%Y-%m-%d %H:%M:%S")
                .unwrap();

        assert_eq!(date.format_pretty_with_reference(&reference_date), "20:56 today");
    }

    #[test]
    fn test_yesterday() {
        let date =
            NaiveDateTime::parse_from_str("2015-09-05 20:56:04", "%Y-%m-%d %H:%M:%S")
                .unwrap();

        let reference_date =
            NaiveDateTime::parse_from_str("2015-09-06 23:56:05", "%Y-%m-%d %H:%M:%S")
                .unwrap();

        assert_eq!(date.format_pretty_with_reference(&reference_date), "20:56 yesterday");
    }

    #[test]
    fn test_yesterday_eom() {
        let date =
            NaiveDateTime::parse_from_str("2015-08-31 20:56:04", "%Y-%m-%d %H:%M:%S")
                .unwrap();

        let reference_date =
            NaiveDateTime::parse_from_str("2015-09-01 23:56:05", "%Y-%m-%d %H:%M:%S")
                .unwrap();

        assert_eq!(date.format_pretty_with_reference(&reference_date), "20:56 yesterday");
    }

    #[test]
    fn this_week() {
        let date =
            NaiveDateTime::parse_from_str("2015-08-31 20:56:04", "%Y-%m-%d %H:%M:%S")
                .unwrap();

        let reference_date =
            NaiveDateTime::parse_from_str("2015-09-06 23:56:05", "%Y-%m-%d %H:%M:%S")
                .unwrap();

        assert_eq!(date.format_pretty_with_reference(&reference_date), "20:56 Monday");
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