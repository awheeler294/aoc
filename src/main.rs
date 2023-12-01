use std::env;
use std::fs;

mod grid;
mod y2015;
mod y2022;
mod y2023;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some(year) = args.get(1) {
        if let Some(day) = args.get(2) {
            println!("year {}", year);
            println!("day {}", day);

            let path = format!("input/{year}/day{day}");
            let data = fs::read_to_string(&path).expect(&format!("Cant read {}", &path));
            let lines = data.trim_end().split('\n').collect::<Vec<&str>>();

            match year.as_str() {
                "2015" => {
                    println!("{}", y2015::run_day(day, &lines))
                }
                "2022" => {
                    println!("{}", y2022::run_day(day, &lines))
                }
                "2023" => {
                    println!("{}", y2023::run_day(day, &lines))
                }
                _ => {
                    println!("Unknown year {}", year);
                }
            }
        } else {
            println!("Usage: year day");
        }
    } else {
        println!("Usage: year day");
    }
}
