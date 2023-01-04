# Pretty Date

A rust library for the simple, friendly, human readable formatting for the chrono NaiveDateTime, for example:
- Within the last ten minutes: `Just now`
- Earlier today: `20:56 Today`
- Earlier in the year: `5 September, 23:56`

# Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
pretty_date = "0.1.2"
```

Basic default usage:
```rust
use chrono::NaiveDateTime;
use pretty_date::pretty_date_formatter::PrettyDateFormatter;

fn main() {
    let date = NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S").unwrap();
    println!("{}", date.format_pretty());
}
```

Advanced usage:
```rust
use chrono::NaiveDateTime;
use pretty_date::pretty_date_format::PrettyDateFormat;
use pretty_date::pretty_date_rule::PrettyDateRule;

fn main() {
    let date = NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S").unwrap();
    let date_format = PrettyDateFormat {
        rules: vec![
            PrettyDateRule::Today,
            PrettyDateRule::ThisYear,
        ],
        default_format: "%-e %B %Y, %H:%M",
    };
    println!("{}", date_format.format_pretty(&date));
}
```

# License

Rand is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT), and
[COPYRIGHT](COPYRIGHT) for details.
