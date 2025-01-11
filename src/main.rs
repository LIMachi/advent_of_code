#![feature(ascii_char)]
#![feature(let_chains)]

#![allow(dead_code)]

use std::error::Error;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use proc_macros::{gen_day_caller, gen_days, gen_mod_imports};

mod p2024;
mod functions;

gen_days!((2024,));
gen_mod_imports!((2024,));
gen_day_caller!((2024,));

fn run_day(year: u32, day: u32) {
    if let Ok(input) = fs::read_to_string(format!("inputs/{year}/{day}.txt")) {
        let (a, b) = day_caller(year, day, input.as_str()).unwrap();
        let mut fa = OpenOptions::new().create(true).write(true).open(format!("outputs/{year}/{day}a.txt")).expect(format!("Cannot create output for day {day} of year {year}").as_str());
        let mut fb = OpenOptions::new().create(true).write(true).open(format!("outputs/{year}/{day}b.txt")).expect(format!("Cannot create output for day {day} of year {year}").as_str());
        fa.write_fmt(format_args!("{a}")).expect(format!("could not write to outputs/{year}/{day}a.txt").as_str());
        fb.write_fmt(format_args!("{b}")).expect(format!("could not write to outputs/{year}/{day}b.txt").as_str());
    }
}

fn setup_inputs_and_outputs(year: usize) {
    fs::create_dir_all(format!("inputs/{year}")).ok(); //since create_dir_all does not have a distinction between "already exists", "missing parent" and "invalid rights", we just ignore the error (the rest of the code will not run anyway if the directory was not created)
    fs::create_dir_all(format!("outputs/{year}")).ok(); //same as above
    for d in 1..=25 {
        OpenOptions::new().create(true).append(true).open(format!("inputs/{year}/{d}.txt")).expect(format!("Cannot create input for day {d} of year {year}").as_str());
    }
}


fn main() {
    setup_inputs_and_outputs(2024);
    run_day(2024, 1);
}