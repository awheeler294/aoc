mod part1;
mod part2;

pub fn solve(input: &[&str]) -> String {
    let part1 = part1::score_game(input);
    let part2 = part2::score_game(input);

    format!(" Part1: {} \n Part2: {}", part1, part2)
}
