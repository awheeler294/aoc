mod day1;
mod day10;
mod day11;
mod day12;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub fn run_day(day: &str, input: &[&str]) -> String {
    match day {
        "1" => day1::solve(&input),
        "2" => day2::solve(&input),
        "3" => day3::solve(&input),
        "4" => day4::solve(&input),
        "5" => day5::solve(&input),
        "6" => day6::solve(&input),
        "7" => day7::solve(&input),
        "8" => day8::solve(&input),
        "9" => day9::solve(&input),
        "10" => day10::solve(&input),
        "11" => day11::solve(&input),
        "12" => day12::solve(&input),
        _ => {
            format!("Unknown day {} ", day)
        }
    }
}
