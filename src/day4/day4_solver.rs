use super::DayPart;

use std::fs;

#[derive(PartialEq, Debug)]
pub enum Day4Error {
    FileReadingError,
    IncorrectCharacter(char),
    MapOutOfBound,
}

#[derive(Debug)]
enum Tile {
    Empty,
    Roll,
}

impl TryFrom<char> for Tile {
    type Error = Day4Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Tile::Empty),
            '@' => Ok(Tile::Roll),
            _ => Err(Day4Error::IncorrectCharacter(c)),
        }
    }
}

#[derive(Debug)]
struct Map {
    lines: Vec<Vec<Tile>>,
}

impl Map {
    fn new(input: &str) -> Result<Map, Day4Error> {
        let lines = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(Tile::try_from)
                    .collect::<Result<Vec<Tile>, Day4Error>>()
            })
            .collect::<Result<Vec<Vec<Tile>>, Day4Error>>()?;
        Ok(Map { lines })
    }

    fn get(&self, col: usize, line: usize) -> Result<&Tile, Day4Error> {
        self.lines
            .get(line)
            .ok_or(Day4Error::MapOutOfBound)?
            .get(col)
            .ok_or(Day4Error::MapOutOfBound)
    }
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl Direction {
    fn get_shift(&self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
            Direction::NorthEast => (1, -1),
            Direction::NorthWest => (-1, -1),
            Direction::SouthEast => (1, 1),
            Direction::SouthWest => (-1, 1),
        }
    }

    fn shift(&self, col: i32, line: i32) -> (i32, i32) {
        let (shift_col, shift_line) = self.get_shift();
        (col + shift_col, line + shift_line)
    }
}

fn is_roll_accssible(map: &Map, col: usize, line: usize) -> bool {
    let mut adjacent_rolls = 0;
    let all_directions = vec![
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
        Direction::NorthEast,
        Direction::NorthWest,
        Direction::SouthEast,
        Direction::SouthWest,
    ];
    for direction in all_directions {
        let (cell_col, cell_line) = direction.shift(col as i32, line as i32);
        adjacent_rolls += match map.get(cell_col as usize, cell_line as usize) {
            Ok(t) => match t {
                Tile::Empty => 0,
                Tile::Roll => 1,
            },
            Err(_) => 0,
        }
    }

    adjacent_rolls < 4
}

fn count_accessible_rolls(map: &Map) -> u64 {
    let mut counter: u64 = 0;
    for (idx_line, line) in map.lines.iter().enumerate() {
        for (idx_col, tile) in line.iter().enumerate() {
            counter += match tile {
                Tile::Empty => 0,
                Tile::Roll if is_roll_accssible(map, idx_col, idx_line) => 1,
                Tile::Roll => 0,
            }
        }
    }
    counter
}

pub fn solve_day4(path: &str, _part: DayPart) -> Result<u64, Day4Error> {
    let file_content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(_) => return Err(Day4Error::FileReadingError),
    };
    let map = Map::new(file_content.as_str())?;
    Ok(count_accessible_rolls(&map))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day4_1_example() {
        // Given
        let file_path = "inputs/day4_example.txt";

        // When
        let result = solve_day4(file_path, DayPart::Part1);

        // Then
        assert_eq!(result, Ok(13));
    }
}
