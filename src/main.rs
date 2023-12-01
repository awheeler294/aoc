use std::env;
use std::fs;

use clap::Parser;

mod grid;
mod y2015;
mod y2022;
mod y2023;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    year: String,
    day: String,
}

fn main() {
    let args = Args::parse();

    println!("year {}", args.year);
    println!("day {}", args.day);

    let path = format!("input/{}/day{}", args.year, args.day);
    let data = fs::read_to_string(&path).expect(&format!("Cant read {}", &path));
    let lines = data.trim_end().split('\n').collect::<Vec<&str>>();

    match args.year.as_str() {
        "2015" => {
            println!("{}", y2015::run_day(&args.day, &lines))
        }
        "2022" => {
            println!("{}", y2022::run_day(&args.day, &lines))
        }
        "2023" => {
            println!("{}", y2023::run_day(&args.day, &lines))
        }
        _ => {
            println!("Unknown year {}", args.year);
        }
    }
}
