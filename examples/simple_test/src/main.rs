use carbono::Carbono;

fn main() {
    let carbono = Carbono::now(); // 1999-12-31 23:59:59

    println!("{}", carbono); // 1999-12-31 23:59:59 UTC

    assert_eq!(carbono.timestamp(), 946684799);

    assert_eq!(carbono.rfc3339(), "1999-12-31T23:59:59+00:00");
    assert_eq!(carbono.rfc2822(), "Fri, 31 Dec 1999 23:59:59 +0000");

    assert_eq!(carbono.year(), 1999);
    assert_eq!(carbono.month(), 12);
    assert_eq!(carbono.day(), 31);
    assert_eq!(carbono.hour(), 23);
    assert_eq!(carbono.minute(), 59);
    assert_eq!(carbono.second(), 59);

    assert_eq!(carbono.date(), "1999-12-31");
    assert_eq!(carbono.time(), "23:59:59");

    assert_eq!(format!("{:?}", carbono.iso_week()), "1999-W52");

    assert_eq!(carbono.weekday(), 4); // Friday

    assert_eq!(carbono.is_thursday(), false);
    assert_eq!(carbono.is_friday(), true);
    assert_eq!(carbono.is_saturday(), false);

    assert_eq!(carbono.is_past(), true);
    assert_eq!(carbono.is_future(), false);

    assert_eq!(carbono.is_today(), true); // 1999-12-31

    assert_eq!(carbono.add_day().is_today(), false); // 2000-01-01

    assert_eq!(carbono.add_year().add_month().date(), "2001-01-31");
}
