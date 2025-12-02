#!/bin/bash
set -euo pipefail
IFS=$'\n\t'

year=$1
day=$2
session="$AOC_SESSION"

pushd $HOME/workspace/aoc/

inputs_dir="input/$year/"
mkdir -p "$inputs_dir"

curl https://adventofcode.com/"$year"/day/"$day"/input \
  -H 'authority: adventofcode.com' \
  -H 'accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9' \
  -H 'accept-language: en-US,en;q=0.9' \
  -H 'cache-control: max-age=0' \
  -H "cookie: session=$session" \
  -H 'dnt: 1' \
  -H "referer: https://adventofcode.com/$year/day/$day" \
  -H 'sec-fetch-dest: document' \
  -H 'sec-fetch-mode: navigate' \
  -H 'sec-fetch-site: same-origin' \
  -H 'sec-fetch-user: ?1' \
  -H 'sec-gpc: 1' \
  -H 'upgrade-insecure-requests: 1' \
  --compressed \
  -o "$inputs_dir"/day"$day"

src_dir="src/y$year"
if [ ! -d "$src_dir" ]
then

   mkdir -p "$src_dir"

   for d in {1..25}
   do
      src_file="$src_dir/day$d.rs"
      if [ ! -f "$src_file" ]
      then
         cat > "$src_file" << EOF
pub fn solve(input: &[&str]) -> String {
   let part_1 = solve_part_1(input);
   let part_2 = solve_part_2(input);

   format!(" Part1: {part_1} \n Part2: {part_2}")
}

fn solve_part_1(input: &[&str]) -> usize {
   input.len() - input.len()
}

fn solve_part_2(input: &[&str]) -> usize {
   input.len() - input.len()
}

#[cfg(test)]
mod tests {
    
   use super::*;
   use pretty_assertions::assert_eq;

   #[test]
   fn test_solve_part_1() {
      let input = [];

      let expected = 0;

      let actual = solve_part_1(&input);

      assert_eq!(actual, expected);
   }

   #[test]
   fn test_solve_part_2() {
      let input = [];

      let expected = 0;

      let actual = solve_part_2(&input);

      assert_eq!(actual, expected);
   }
}
EOF
      fi
   done



   cat > "$src_dir/mod.rs" << EOF
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
EOF
fi

popd
