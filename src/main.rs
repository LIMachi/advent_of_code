#![feature(ascii_char)]
#![feature(let_chains)]

#![allow(dead_code)]

use std::env::args;
use std::fs;
use std::fs::{OpenOptions};
use std::io::{Write};
use proc_macros::advent_of_code_setup;
use utils::*;

mod p2024;
mod functions;

advent_of_code_setup!();

fn run_day(year: u32, day: u32) {
    if let Ok(input) = fs::read_to_string(format!("inputs/{year}/{day}.txt")) {
        let (a, b) = day_caller(year, day, input.as_str()).unwrap();
        println!("{year}/12/{day} -> a: {}, b: {}", a, b);
        let mut fa = OpenOptions::new().create(true).write(true).open(format!("outputs/{year}/{day}a.txt")).expect(format!("Cannot create output for day {day} of year {year}").as_str());
        let mut fb = OpenOptions::new().create(true).write(true).open(format!("outputs/{year}/{day}b.txt")).expect(format!("Cannot create output for day {day} of year {year}").as_str());
        fa.write_fmt(format_args!("{a}")).expect(format!("could not write to outputs/{year}/{day}a.txt").as_str());
        fb.write_fmt(format_args!("{b}")).expect(format!("could not write to outputs/{year}/{day}b.txt").as_str());
    }
}

fn parse_args(args: Vec<String>) -> Option<(u32, u32, u32)> {
    if args.len() >= 2 {
        let year: u32 = args[1].parse().ok()?;
        if args.len() >= 3 {
            if let Ok(day) = args[2].parse() {
                return Some((year, 12, day))
            }
        }
        Some((year, 12, 31))
    } else {
        get_advent_year_month_day()
    }
}

fn main() {
    if let Some((year, month, day)) = parse_args(args().collect()) {
        if month == 12 && day <= 25 {
            run_day(year, day);
        } else {
            for i in 1..=25 {
                run_day(year, i);
            }
        }
    }
}