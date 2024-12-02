mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
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
        "13" => day13::solve(&input),
        "14" => day14::solve(&input),
        "15" => day15::solve(&input),
        "16" => day16::solve(&input),
        "17" => day17::solve(&input),
        "18" => day18::solve(&input),
        "19" => day19::solve(&input),
        "20" => day20::solve(&input),
        "21" => day21::solve(&input),
        "22" => day22::solve(&input),
        "23" => day23::solve(&input),
        "24" => day24::solve(&input),
        "25" => day25::solve(&input),
        _ => {
            format!("Unknown day {} ", day)
        }
    }
}
