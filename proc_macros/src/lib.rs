use proc_macro::TokenStream;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use quote::quote;
use syn::{parse_macro_input, Expr, ExprLit, ExprTuple, Ident, Index};
use proc_macro2::Span;

//FIXME: could use a real range instead
fn parse_year_range(input: ExprTuple) -> (usize, usize) {
    if input.elems.len() == 1 {
        if let Expr::Lit(ExprLit { lit: syn::Lit::Int(lit), .. }) = &input.elems[0] {
            let lit = lit.base10_parse::<usize>().expect("failed to parse year as an integer");
            (lit, lit)
        } else {
            panic!("year is not an integer");
        }
    } else if input.elems.len() == 2 {
        match (&input.elems[0], &input.elems[1]) {
            (Expr::Lit(ExprLit { lit: syn::Lit::Int(lit1), .. }), Expr::Lit(ExprLit { lit: syn::Lit::Int(lit2), .. })) => {
                let lit1 = lit1.base10_parse::<usize>().expect("failed to parse year.0 as an integer");
                let lit2 = lit2.base10_parse::<usize>().expect("failed to parse year.1 as an integer");
                (lit1, lit2)
            }
            _ => {
                panic!("year is not a range (pair) of integers")
            }
        }
    } else {
        panic!("year should be either 1 (single) or 2 (range inclusive) integers")
    }
}

fn setup_functions(year: usize) {
    fs::create_dir_all(format!("src/functions/y{year}")).ok();
    let mut m = OpenOptions::new().create(true).write(true).open(format!("src/functions/y{year}/mod.rs")).expect(format!("Cannot create mod file for year {year}").as_str());
    for d in 1..=25 {
        m.write_fmt(format_args!("pub mod d{d};\n")).expect(format!("Cannot write mod file for year {year}").as_str());
        if let Ok(mut f) = OpenOptions::new().create_new(true).write(true).open(format!("src/functions/y{year}/d{d}.rs")) {
            f.write_fmt(format_args!(
                "pub fn y{year}d{d}a(input: &str) -> Result<String, String> {{\
                \n    Ok(\"\".to_string())\n\
                }}\n\n\
                pub fn y{year}d{d}b(input: &str) -> Result<String, String> {{\
                \n    Ok(\"\".to_string())\n\
                }}\n"
            )).expect(format!("Cannot write function file for day {d} of year {year}").as_str());
        }
    }
}

///input:
///generate a function called day_caller that takes in input a year and day
///and return a result
///pub fn day_caller(year: u32, day: u32, input: &str) -> Result<(String, String), String>
///when called, will actually call the functions:
///y{year}d{day}a and y{year}d{day}b in sequence
///returning unit if everything worked, and a string error if something went wrong
///this macro is expected to be called at the root module, and functions
///are expected to be sorted in year modules and day files (both a and b functions
///in the same day file)
///example: y2024d3a should be at this position: crate::y2024::d3::y2024d3a
#[proc_macro]
pub fn gen_day_caller(input: TokenStream) -> TokenStream {
    let (start, end) = parse_year_range(parse_macro_input!(input as ExprTuple));

    let mut cases = Vec::new();
    for year in start..=end {
        let y = Index::from(year);
        let y_dir = Ident::new(format!("y{year}").as_str(), Span::call_site());
        for day in 1..=25 {
            let d = Index::from(day);
            let d_dir = Ident::new(format!("d{day}").as_str(), Span::call_site());
            let fun_a = Ident::new(format!("y{year}d{day}a").as_str(), Span::call_site());
            let fun_b = Ident::new(format!("y{year}d{day}b").as_str(), Span::call_site());
            cases.push(quote!{
                (#y, #d) => Ok((crate::functions::#y_dir::#d_dir::#fun_a(input)?, crate::functions::#y_dir::#d_dir::#fun_b(input)?)),
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

#[proc_macro]
pub fn gen_mod_imports(input: TokenStream) -> TokenStream {
    let (start, end) = parse_year_range(parse_macro_input!(input as ExprTuple));

    let mut m = OpenOptions::new().create(true).write(true).open("src/functions/mod.rs").expect("Cannot create mod file for functions");

    for year in start..=end {
        m.write_fmt(format_args!("pub mod y{year};\n")).expect("Could not write mod file for functions");
    }

    TokenStream::new()
}

#[proc_macro]
pub fn gen_days(input: TokenStream) -> TokenStream {
    let (start, end) = parse_year_range(parse_macro_input!(input as ExprTuple));
    for year in start..=end {
        setup_functions(year);
    }
    TokenStream::new()
}