# Carbono

[![crates.io](https://img.shields.io/crates/v/carbono.svg?style=flat-square)](https://crates.io/crates/carbono)
![cargo build](https://img.shields.io/github/actions/workflow/status/tjardoo/carbono/rust.yml?style=flat-square)
[![docs.rs](https://img.shields.io/docsrs/carbono?style=flat-square)](https://docs.rs/carbono)
[![crates.io](https://img.shields.io/crates/d/carbono.svg?style=flat-square)](https://crates.io/crates/carbono)

Carbono is a simple Rust API extension for chrono DateTime.

```ini
[dependencies]
carbono = "0.1"
```

## Getting Started

```rust
use carbono::Carbono;

fn main() {
    let carbono = Carbono::now(); // 2022-12-15 12:30:00

    println!("{}", carbono); // 2022-12-15 12:30:00 UTC

    assert_eq!(carbono.timestamp(), 1671107400);

    assert_eq!(carbono.rfc3339(), "2022-12-15T12:30:00+00:00");
    assert_eq!(carbono.rfc2822(), "Thu, 15 Dec 2022 12:30:00 +0000");

    assert_eq!(carbono.year(), 2022);
    assert_eq!(carbono.month(), 12);
    assert_eq!(carbono.day(), 15);
    assert_eq!(carbono.hour(), 12);
    assert_eq!(carbono.minute(), 30);
    assert_eq!(carbono.second(), 0);

    assert_eq!(carbono.datetime(), "2022-12-15 12:30:00");

    assert_eq!(carbono.date(), "2022-12-15");
    assert_eq!(carbono.time(), "12:30:00");

    assert_eq!(format!("{:?}", carbono.iso_week()), "2022-W50");

    assert_eq!(carbono.weekday(), 3); // Thursday

    assert_eq!(carbono.is_wednesday(), false);
    assert_eq!(carbono.is_thursday(), true);
    assert_eq!(carbono.is_friday(), false);

    assert_eq!(carbono.is_past(), true);
    assert_eq!(carbono.is_future(), false);

    assert_eq!(carbono.is_today(), true); // 2022-12-15

    assert_eq!(carbono.add_day().is_today(), false); // 2022-12-16

    assert_eq!(carbono.add_year().add_month().date(), "2024-01-15");

    assert_eq!(carbono.start_year().datetime(), "2022-01-01 00:00:00");
}
```

### Alter the date and/or time

- `add_year()`
- `sub_year()`
- `add_years(years: i32)`
- `sub_years(years: i32)`
- `add_month()`
- `sub_month()`
- `add_months(months: i32)`
- `sub_months(months: i32)`
- `add_week()`
- `sub_week()`
- `add_weeks(weeks: i64)`
- `sub_weeks(weeks: i64)`
- `add_day()`
- `sub_day()`
- `add_days(days: i64)`
- `sub_days(days: i64)`
- `add_hour()`
- `sub_hour()`
- `add_hours(hours: i64)`
- `sub_hours(hours: i64)`
- `add_minute()`
- `sub_minute()`
- `add_minutes(minutes: i64)`
- `sub_minutes(minutes: i64)`
- `add_second()`
- `sub_second()`
- `add_seconds(seconds: i64)`
- `sub_seconds(seconds: i64)`

### Move to the start/end of period

- `start_year`
- `start_month`
- `start_day`
- `start_hour`
- `start_minute`
- `end_year`
- `end_month`
- `end_day`
- `end_hour`
- `end_minute`
