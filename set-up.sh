#!/bin/bash
set -euo pipefail
IFS=$'\n\t'

year=$1
day=$2
session="53616c7465645f5f204d2a879a30f31bb9520ac35cc9682819f5b6cbd3e4d42672ec52ad0a80661aa18b782a956b3a4d6d813798e071612a8080d73892314a0a"

pushd $HOME/workspace/aoc-rs/

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
  -H 'sec-ch-ua: "Chromium";v="106", "Not;A=Brand";v="99"' \
  -H 'sec-ch-ua-mobile: ?0' \
  -H 'sec-ch-ua-platform: "Linux"' \
  -H 'sec-fetch-dest: document' \
  -H 'sec-fetch-mode: navigate' \
  -H 'sec-fetch-site: same-origin' \
  -H 'sec-fetch-user: ?1' \
  -H 'sec-gpc: 1' \
  -H 'upgrade-insecure-requests: 1' \
  -H 'user-agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/106.0.0.0 Safari/537.36' \
  --compressed \
  -o "$inputs_dir"/day"$day"

for d in {1..25}
do
   src_dir="src/y$year"
   src_file="$src_dir/day$d.rs"
   if [ ! -f "$src_file" ]; then
      mkdir -p "$src_dir"
      cat > "$src_file" << EOF
pub fn solve(_input: &[&str]) -> String {
    let part1 = "";
    let part2 = "";

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test_solve() {

    }
}
EOF
   fi
done

popd
