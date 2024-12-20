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
   let part1 = solve_part_1(input);
   let part2 = solve_part_2(input);

   format!(" Part1: {} \n Part2: {}", part1, part2)
}

fn solve_part_1(input: &[&str]) -> usize {
   input.len()
}

fn solve_part_2(input: &[&str]) -> usize {
   input.len()
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
fi

popd
