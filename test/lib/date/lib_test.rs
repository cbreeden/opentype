#![feature(macro_rules)]

extern crate date;
use date::Date;

macro_rules! date(
    ($year:expr, $month:expr, $day:expr) => (
        Date { year: $year, month: $month, day: $day }
    )
)

#[test]
fn eq_test() {
    assert_eq!(date!(2014, 08, 19), date!(2014, 08, 19));
}

#[test]
fn ord_test() {
    assert!(date!(2014, 08, 19) < date!(2014, 08, 20));
    assert!(date!(2014, 08, 19) > date!(2014, 08, 18));
    assert!(date!(2014, 08, 19) < date!(2014, 09, 19));
    assert!(date!(2014, 08, 19) > date!(2014, 07, 19));
    assert!(date!(2014, 08, 19) < date!(2015, 08, 19));
    assert!(date!(2014, 08, 19) > date!(2013, 08, 19));
}

#[test]
fn at_since_1904_test() {
    assert_eq!(Date::at_since_1904(         0), date!(1904, 01, 01));
    assert_eq!(Date::at_since_1904(   2678399), date!(1904, 01, 31));
    assert_eq!(Date::at_since_1904(   2678400), date!(1904, 02, 01));
    assert_eq!(Date::at_since_1904(   5184000), date!(1904, 03, 01));
    assert_eq!(Date::at_since_1904(3491078399), date!(2014, 08, 16));
    assert_eq!(Date::at_since_1904(3491078400), date!(2014, 08, 17));
}