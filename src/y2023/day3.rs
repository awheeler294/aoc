use crate::grid::{Grid, GridDirection, Point};

pub fn solve(input: &[&str]) -> String {
    let part1 = find_part_numbers(input).sum::<u32>();
    let part2 = find_gear_ratios(input).sum::<u32>();

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

fn find_part_numbers(input: &[&str]) -> impl Iterator<Item = u32> {
    let grid = Grid::parse_char(input);

    let mut part_numbers = vec![];

    let mut digit_buff = String::new();
    let mut part_symbol_found = false;

    for (i, ch) in grid.iter().enumerate() {
        let point = grid.idx_point(i);

        if ch.is_digit(10) {
            digit_buff.push(*ch);

            if part_symbol_found == false {
                for val in GridDirection::all()
                    .iter()
                    .filter_map(|direction| grid.get_direction(&point, *direction))
                {
                    if *val != '.' && val.is_ascii_punctuation() {
                        part_symbol_found = true;
                    }
                }
            }
        }

        if ch.is_digit(10) == false || grid.get_direction(&point, GridDirection::Right).is_none() {
            if part_symbol_found {
                part_numbers.push(digit_buff.parse::<u32>().unwrap());
            }

            part_symbol_found = false;
            digit_buff.truncate(0);
        }
    }

    if part_symbol_found {
        part_numbers.push(digit_buff.parse::<u32>().unwrap());
    }

    part_numbers.into_iter()
}

const GEAR_SYMBOL: char = '*';
fn find_gear_ratios(input: &[&str]) -> impl Iterator<Item = u32> {
    let grid = Grid::parse_char(input);

    let mut gear_ratios = vec![];

    for (i, ch) in grid.iter().enumerate() {
        let point = grid.idx_point(i);

        if *ch == GEAR_SYMBOL {
            if let Some(part_numbers) = find_gear_part_numbers(&point, &grid) {
                gear_ratios.push(part_numbers.iter().product());
            }
        }
    }

    gear_ratios.into_iter()
}

fn find_gear_part_numbers(point: &Point, grid: &Grid<char>) -> Option<[u32; 2]> {
    let mut visited = Grid::new(grid.width, grid.height, false);
    let mut part_numbers = vec![];

    for (point, _) in GridDirection::all()
        .iter()
        .filter_map(|direction| grid.enumerate_direction(&point, *direction))
    {
        if let Some(number) = find_number(&point, &grid, &mut visited) {
            part_numbers.push(number);
        }
    }

    if part_numbers.len() == 2 {
        Some([part_numbers[0], part_numbers[1]])
    } else {
        None
    }
}

fn find_number(point: &Point, grid: &Grid<char>, visited: &mut Grid<bool>) -> Option<u32> {
    assert_eq!(
        grid.width, visited.width,
        "grid and visited must have the same dimensions"
    );
    assert_eq!(
        grid.height, visited.height,
        "grid and visited must have the same dimensions"
    );

    if *visited.get(point).unwrap() {
        // we've already visited this number
        return None;
    }

    if let Some(val) = grid.get(point) {
        if val.is_digit(10) == false {
            return None;
        }

        let mut cursor = *point;
        // find left-most digit
        while let Some((left, val)) = grid.enumerate_direction(&cursor, GridDirection::Left) {
            if *visited.get(&left).unwrap() {
                // we've already visited this number
                return None;
            }

            if val.is_digit(10) == false {
                // cursor is currently at the left-most digit
                break;
            }

            cursor = left;
        }

        let mut digit_buff = String::new();
        if let Some(digit) = grid.get(&cursor) {
            digit_buff.push(*digit);
            visited.set(&cursor, true).unwrap();
        } else {
            return None;
        }

        while let Some((right, val)) = grid.enumerate_direction(&cursor, GridDirection::Right) {
            if *visited.get(&right).unwrap() {
                // we've already visited this number
                return None;
            }

            if val.is_digit(10) {
                digit_buff.push(*val);
                visited.set(&right, true).unwrap();
            } else {
                // we've reached the end of this number
                return digit_buff.parse().ok();
            }

            cursor = right;
        }

        return digit_buff.parse().ok();
    }

    None
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_find_part_numbers() {
        #[rustfmt::skip]
        let input = [
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ];

        let expected = vec![467, 35, 633, 617, 592, 755, 664, 598];
        let actual = find_part_numbers(&input).collect::<Vec<u32>>();

        assert_eq!(actual, expected);

        assert_eq!(actual.iter().sum::<u32>(), 4361);
    }

    #[test]
    fn test_adjacent_part_numbers() {
        #[rustfmt::skip]
        let input = [
            "467*114...",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ];

        let expected = vec![467, 114, 35, 633, 617, 592, 755, 664, 598];
        let actual = find_part_numbers(&input).collect::<Vec<u32>>();

        assert_eq!(actual, expected);

        assert_eq!(actual.iter().sum::<u32>(), 4361 + 114);
    }

    #[test]
    fn test_part_numbers_do_not_span_linebreaks() {
        #[rustfmt::skip]
        let input = [
            "467..114..",
            "...*......",
            "..35..633.",
            "......#123",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ];

        let expected = vec![467, 35, 633, 123, 617, 592, 755, 664, 598];
        let actual = find_part_numbers(&input).collect::<Vec<u32>>();

        assert_eq!(actual, expected);

        assert_eq!(actual.iter().sum::<u32>(), 4361 + 123);
    }

    #[test]
    fn test_find_gear_ratios() {
        #[rustfmt::skip]
        let input = [
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ];

        let expected = vec![16345, 451490];
        let actual = find_gear_ratios(&input).collect::<Vec<u32>>();

        assert_eq!(actual, expected);

        assert_eq!(actual.iter().sum::<u32>(), 467835);
    }

    #[test]
    fn test_find_number() {
        #[rustfmt::skip]
        let input = [
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58*",
            "..592..111",
            "......755.",
            "...$.*....",
            ".664.598..",
        ];

        let grid = Grid::parse_char(&input);

        let point = Point::new(2, 0);
        let mut visited = Grid::new(grid.width, grid.height, false);

        let expected = Some(467);
        let actual = find_number(&point, &grid, &mut visited);
        assert_eq!(actual, expected);

        let point = Point::new(9, 6);
        let mut visited = Grid::new(grid.width, grid.height, false);

        let expected = Some(111);
        let actual = find_number(&point, &grid, &mut visited);
        assert_eq!(actual, expected);

        let point = Point::new(8, 6);
        let mut visited = Grid::new(grid.width, grid.height, false);

        let expected = Some(111);
        let actual = find_number(&point, &grid, &mut visited);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_find_gear_part_numbers() {
        #[rustfmt::skip]
        let input = [
            "467...114.",
            "...*.....*",
            "..35..633.",
            "......#...",
            "617*1.....",
            ".....+.58*",
            "..592..111",
            "......755.",
            "...$.*....",
            ".664.598..",
        ];

        let grid = Grid::parse_char(&input);

        let point = Point::new(3, 1);
        let expected = Some([467, 35]);
        let actual = find_gear_part_numbers(&point, &grid);

        assert_eq!(actual.unwrap().sort(), expected.unwrap().sort());

        let point = Point::new(9, 1);
        let expected = Some([114, 633]);
        let actual = find_gear_part_numbers(&point, &grid);

        assert_eq!(actual.unwrap().sort(), expected.unwrap().sort());

        let point = Point::new(3, 4);
        let expected = Some([617, 1]);
        let actual = find_gear_part_numbers(&point, &grid);

        assert_eq!(actual.unwrap().sort(), expected.unwrap().sort());

        let point = Point::new(9, 5);
        let expected = Some([58, 111]);
        let actual = find_gear_part_numbers(&point, &grid);

        assert_eq!(actual.unwrap().sort(), expected.unwrap().sort());
    }
}
