use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use chrono::prelude::*;

pub const FIRST_YEAR: u32 = 2024; //true first year: 2015

pub fn get_year_month_day() -> (u32, u32, u32) {
    let date = Utc::now().with_timezone(&FixedOffset::west_opt(5 * 3600).unwrap());
    (date.year() as u32, date.month(), date.day())
}

pub fn get_advent_year_month_day() -> Option<(u32, u32, u32)> {
    let (year, month, _day) = get_year_month_day();
    let year = if month < 12 { year - 1 } else { year };
    if year < FIRST_YEAR { None } else { Some((year, month, _day)) }
}

pub fn setup_inputs_and_outputs(year: u32) {
    fs::create_dir_all(format!("inputs/{year}")).ok(); //since create_dir_all does not have a distinction between "already exists", "missing parent" and "invalid rights", we just ignore the error (the rest of the code will not run anyway if the directory was not created)
    fs::create_dir_all(format!("outputs/{year}")).ok(); //same as above
    for d in 1..=25 {
        OpenOptions::new().create(true).append(true).open(format!("inputs/{year}/{d}.txt")).expect(format!("Cannot create input for day {d} of year {year}").as_str());
    }
}

pub fn setup_functions(year: u32) {
    fs::create_dir_all(format!("src/functions/y{year}")).ok();
    let mut m = OpenOptions::new().create(true).write(true).open(format!("src/functions/y{year}/mod.rs")).expect(format!("Cannot create mod file for year {year}").as_str());
    for d in 1..=25 {
        m.write_fmt(format_args!("pub mod d{d};\n")).expect(format!("Cannot write mod file for year {year}").as_str());
        if let Ok(mut f) = OpenOptions::new().create_new(true).write(true).open(format!("src/functions/y{year}/d{d}.rs")) {
            f.write_fmt(format_args!(
                "pub fn y{year}d{d}a(_input: &str) -> Result<String, String> {{\
                \n    Ok(\"\".to_string())\n\
                }}\n\n\
                pub fn y{year}d{d}b(_input: &str) -> Result<String, String> {{\
                \n    Ok(\"\".to_string())\n\
                }}\n"
            )).expect(format!("Cannot write function file for day {d} of year {year}").as_str());
        }
    }
}