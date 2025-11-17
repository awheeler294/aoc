use std::collections::HashSet;

use crate::grid::{Grid, GridDirection, Point};

pub fn solve(input: &[&str]) -> String {
   let part1 = solve_part_1(input);
   let part2 = solve_part_2(input);

   format!(" Part1: {} \n Part2: {}", part1, part2)
}

#[derive(Clone, Debug)]
struct Guard {
   position: Point,
   direction: GridDirection,
}

impl Guard {
   fn new(position: Point) -> Self {
      Self {
         position,
         direction: GridDirection::Up,
      }
   }

   fn patrol(&mut self, map: &Grid<char>) -> usize {
      let mut visited = HashSet::new();

      visited.insert(self.position);

      let mut next_position = map.get_direction(&self.position, self.direction);

      while next_position.is_some() {
         while next_position == Some(&'#') {

            self.turn_right();

            next_position = map.get_direction(&self.position, self.direction);

            if next_position.is_none() {
               return visited.len();
            }
         }

         self.position = self.position + self.direction;

         visited.insert(self.position);

         next_position = map.get_direction(&self.position, self.direction);

      }

      visited.len()
   }

   fn find_loops(&mut self, map: &Grid<char>) -> usize {
      let mut visited = HashSet::new();

      let mut next_position = map.get_direction(&self.position, self.direction);

      while next_position.is_some() {
         while next_position == Some(&'#') {

            self.turn_right();

            next_position = map.get_direction(&self.position, self.direction);

            if next_position.is_none() {
               return visited.len();
            }
         }

         if self.can_form_loop(map) {
            visited.insert(self.position + self.direction);
         }

         self.position = self.position + self.direction;

         next_position = map.get_direction(&self.position, self.direction);

      }

      // dbg!({
      //    let mut grid = map.clone();
      //    for p in visited.iter() {
      //       grid.set(p, '0').unwrap();
      //    }
      //
      //    grid
      // });

      visited.len()

   }

   fn can_form_loop(&self, map: &Grid<char>) -> bool {
      if map.get_direction(&self.position, self.direction) != Some(&'.') {
         return false;
      }

      let mut map = map.clone();
      // let mut clone = self.clone();

      if map.set(&(self.position + self.direction), '#').is_ok() {
         
         let mut visited = HashSet::new();
         let mut current_position = self.position;
         let mut current_direction = GridDirection::Up;
         // let mut path = HashSet::new();

         // let mut next_position = map.get_direction(&clone.position, clone.direction);

         loop {
            // Try to move our current direction. If we can't, turn right.
            match map.get_direction(&current_position, current_direction) {
               Some('#') => {
                   current_direction = current_direction.turn_clockwise_90();
               }
               None => return false,
               _ => {
                   current_position = current_position + current_direction;
               }
            }
            if visited.contains(&(current_position, current_direction)) {
                return true;
            }
            visited.insert((current_position, current_direction));
         }
         // while next_position.is_some() {
         //    if next_position == Some(&'#'){
         //       clone.turn_right_90();
         //    }
         //
         //    if visited.contains(&(clone.position, clone.direction)) {
         //
         //       // dbg!({
         //       //    let mut grid = map.clone();
         //       //    for (p, _) in path.iter() {
         //       //       grid.set(p, '+').unwrap();
         //       //    }
         //       //    for (p, d) in visited.iter() {
         //       //       let c = match d {
         //       //          GridDirection::Up => '^',
         //       //          GridDirection::Down => 'v',
         //       //          GridDirection::Left => '<',
         //       //          GridDirection::Right => '>',
         //       //          _ => unreachable!(),
         //       //       };
         //       //       grid.set(p, c).unwrap();
         //       //    }
         //       //    // grid.set(&start.0, 'G').unwrap();
         //       //
         //       //    grid
         //       // });
         //
         //       return true;
         //    }
         //
         //    visited.insert((clone.position, clone.direction));
         //
         //
         //    if map.get_direction(&clone.position, clone.direction).is_none() {
         //       return false;
         //    }
         //    
         //    path.insert((clone.position, clone.direction));
         //
         //    clone.position = clone.position + clone.direction;
         //
         //    next_position = map.get_direction(&clone.position, clone.direction);
         //
         // }
      }

      false
   }

   fn turn_right(&mut self) {
      self.direction = self.direction.turn_clockwise_90();
   }
    
}

fn solve_part_1(input: &[&str]) -> usize {
   let map = Grid::parse_char(input);

   let guard_position = {
      let (idx, _) = map.iter().enumerate().filter(|(_, ch)| **ch == '^').next().unwrap();
      map.idx_point(idx)
   };

   let mut guard = Guard::new(guard_position);

   guard.patrol(&map)
}

fn solve_part_2(input: &[&str]) -> usize {
   let map = Grid::parse_char(input);

   let guard_position = {
      let (idx, _) = map.iter().enumerate().filter(|(_, ch)| **ch == '^').next().unwrap();
      map.idx_point(idx)
   };

   let mut guard = Guard::new(guard_position);

   guard.find_loops(&map)
}

#[cfg(test)]
mod tests {
    
   use super::*;
   use pretty_assertions::assert_eq;

   #[test]
   fn test_solve_part_1() {
      let input = [
         "....#.....",
         ".........#",
         "..........",
         "..#.......",
         ".......#..",
         "..........",
         ".#..^.....",
         "........#.",
         "#.........",
         "......#...",
      ];

      let expected = 41;

      let actual = solve_part_1(&input);

      assert_eq!(actual, expected);
   }

   #[test]
   fn test_solve_part_2() {
      let input = [
         "....#.....",
         ".........#",
         "..........",
         "..#.......",
         ".......#..",
         "..........",
         ".#..^.....",
         "........#.",
         "#.........",
         "......#...",
      ];

      let expected = 6;

      let actual = solve_part_2(&input);

      assert_eq!(actual, expected);
      assert!(false);
   }
}
