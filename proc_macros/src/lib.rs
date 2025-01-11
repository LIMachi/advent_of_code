use proc_macro::TokenStream;
use std::fs::OpenOptions;
use std::io::Write;
use quote::quote;
use syn::{Ident, Index};
use proc_macro2::Span;
use utils::{FIRST_YEAR, get_advent_year_month_day, setup_functions, setup_inputs_and_outputs};

///usage: make sure you have a module 'functions' with nothing inside but a mod.rs file inside
///all the contents of the 'functions' module will be updated automatically and mod.rs will expose sub-modules for each years
///just import the 'functions' module before calling the advent_of_code_setup!() macro
///folders for the inputs and outputs will be generated too, just copy-paste the inputs from adventofcode.com inside the correct files and you are good to go
#[proc_macro]
pub fn advent_of_code_setup(_input: TokenStream) -> TokenStream {
    let year = if let Some(t) = get_advent_year_month_day() {
        t.0
    } else {
        return TokenStream::new();
    };

    let mut f = OpenOptions::new().create(true).write(true).open("src/functions/mod.rs").expect("Cannot create mod file for years");

    let mut cases = Vec::new();
    for y in FIRST_YEAR..=year {
        setup_functions(y);
        setup_inputs_and_outputs(y);
        let y_i = Index::from(y as usize);
        let folder = Ident::new(format!("y{y}").as_str(), Span::call_site());
        f.write_fmt(format_args!("pub mod y{y};\n")).expect("Cannot write mod file for years");
        for day in 1..=25 {
            let d = Index::from(day);
            let d_dir = Ident::new(format!("d{day}").as_str(), Span::call_site());
            let fun_a = Ident::new(format!("y{y}d{day}a").as_str(), Span::call_site());
            let fun_b = Ident::new(format!("y{y}d{day}b").as_str(), Span::call_site());
            cases.push(quote!{
                (#y_i, #d) => Ok((crate::functions::#folder::#d_dir::#fun_a(input)?, crate::functions::#folder::#d_dir::#fun_b(input)?)),
            })
        }
    }

    TokenStream::from(quote!{
        pub fn day_caller(year: u32, day: u32, input: &str) -> Result<(String, String), String> {
            match (year, day) {
                #(#cases)*
                _ => {
                    Err(format!("Invalid year/day pair: {year} {day}"))
                }
            }
        }
    })
}