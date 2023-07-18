//! Counting Sundays
//!
//! You are given the following information,
//! but you may prefer to do some research for yourself.
//!
//! - 1 Jan 1900 was a Monday.
//! - Thirty days has September,
//!   April, June and November.
//!   All the rest have thirty-one,
//!   Saving February alone,
//!   Which has twenty-eight, rain or shine.
//!   And on leap years, twenty-nine.
//! - A leap year occurs on any year evenly divisible by 4,
//!   but not on a century unless it is divisible by 400.
//!
//! How many Sundays fell on the first of the month
//! during the twentieth century (1 Jan 1901 to 31 Dec 2000)?
use super::Problem;

crate::base_problem!(171, "Counting Sundays");

fn get_result_problem() -> i64 {
    let monday = 2;
    let mut result = 0;
    let mut week_day = (((monday - 1) + 365) % 7) + 1;
    let mut year = 1901;

    while year < 2001 {
        let mut month = 1;
        while month < 13 {
            let num_days = match month {
                1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
                4 | 6 | 9 | 11 => 30,
                _ => {
                    if is_leap_year(year) {
                        29
                    } else {
                        28
                    }
                }
            };

            if week_day == 1 {
                result += 1;
            }

            week_day = (((week_day - 1) + num_days) % 7) + 1;

            month += 1;
        }
        year += 1;
    }

    result as i64
}

fn is_leap_year(year: i32) -> bool {
    if year % 4 == 0 {
        if year % 100 == 0 {
            year % 400 == 0
        } else {
            true
        }
    } else {
        false
    }
}
