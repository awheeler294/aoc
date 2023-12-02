pub fn solve(input: &[&str]) -> String {
    let part1 = count_valid_games(input);
    let part2 = calculate_power(input);

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

const MAX_RED: usize = 12;
const MAX_GREEN: usize = 13;
const MAX_BLUE: usize = 14;

fn count_valid_games(lines: &[&str]) -> usize {
    lines
        .iter()
        .map(|line| GameRecord::from(*line))
        .filter_map(|game| {
            if game.is_valid(MAX_RED, MAX_GREEN, MAX_BLUE) {
                Some(game.id)
            } else {
                None
            }
        })
        .sum()
}

fn calculate_power(lines: &[&str]) -> usize {
    lines
        .iter()
        .map(|line| GameRecord::from(*line))
        .map(|game| game.game_power())
        .sum()
}

#[derive(Debug)]
struct CubeCounts {
    red: usize,
    green: usize,
    blue: usize,
}

#[derive(Debug)]
struct GameRecord {
    id: usize,

    counts: Vec<CubeCounts>,
}

impl GameRecord {
    fn is_valid(&self, red: usize, green: usize, blue: usize) -> bool {
        for count in &self.counts {
            if count.red > red || count.green > green || count.blue > blue {
                return false;
            }
        }

        true
    }

    fn min_red(&self) -> usize {
        self.counts
            .iter()
            .map(|count| count.red)
            .max()
            .expect(&format!("No red counts found in {:?}", self.counts))
    }

    fn min_green(&self) -> usize {
        self.counts
            .iter()
            .map(|count| count.green)
            .max()
            .expect(&format!("No green counts found in {:?}", self.counts))
    }

    fn min_blue(&self) -> usize {
        self.counts
            .iter()
            .map(|count| count.blue)
            .max()
            .expect(&format!("No blue counts found in {:?}", self.counts))
    }

    fn game_power(&self) -> usize {
        self.min_red() * self.min_green() * self.min_blue()
    }
}

impl From<&str> for GameRecord {
    fn from(input: &str) -> Self {
        let mut id = 0;
        let mut counts = Vec::new();
        
        let mut input_iter = input.split(' ');

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        let mut prev_token = "";

        while let Some(token) = input_iter.next() {
            match token.trim_end_matches([',', ';']) {
                "Game" => {
                    let id_token = input_iter.next().expect("Could not parse id from: {token}");
                    id = id_token
                        .strip_suffix(':')
                        .expect("Error parsing ':' from `{id_token}`")
                        .parse::<usize>().expect("Could not parse `{s}` as usize");
                },

                "red" => red = prev_token.parse::<usize>().expect("Could not parse `{prev_token}` as usize"),

                "green" => green = prev_token.parse::<usize>().expect("Could not parse `{prev_token}` as usize"),

                "blue" => blue = prev_token.parse::<usize>().expect("Could not parse `{prev_token}` as usize"),

                _ => {},
            };

            if token.ends_with(";") {
                counts.push(CubeCounts { red, green, blue });
            }

            prev_token = token;
        }

        counts.push(CubeCounts { red, green, blue });

        Self { id, counts }
    }
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_count_valid_games() {
        let input = [
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];

        let expected = 8;
        let actual = count_valid_games(&input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_calculate_power() {
        let input = [
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];

        let expected = 2286;
        let actual = calculate_power(&input);

        assert_eq!(actual, expected);

    }
}
