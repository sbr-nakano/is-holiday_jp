mod calender;

use chrono::{Local};
use crate::calender::holiday::holiday::{Holidays, is_holiday};
use std::path::Path;
use std::process::exit;

fn main() {
    let today = Local::now().naive_local().date();

    let holidays = Holidays::get_from_holiday_jp(Path::new("./res/holidays.yml"));

    if holidays.is_ok() {
        if is_holiday(today, holidays.unwrap()) {
            exit(1)
        } else {
            exit(0)
        }
    } else {
        exit(-1)
    }
}