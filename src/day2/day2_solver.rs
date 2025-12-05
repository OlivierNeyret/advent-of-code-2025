use super::DayPart;
use itertools::Itertools;
use std::{fs, num::ParseIntError};

#[derive(PartialEq, Debug)]
pub enum Day2Error {
    FileReadingError,
    IllformedRange,
    IllformedId(ParseIntError),
}

impl From<ParseIntError> for Day2Error {
    fn from(err: ParseIntError) -> Self {
        Day2Error::IllformedId(err)
    }
}

fn is_id_invalid(id: &str, part: &DayPart) -> bool {
    match part {
        DayPart::Part1 => {
            let (left, right) = id.split_at(id.len() / 2);
            left == right
        }
        DayPart::Part2 => {
            let id_len = id.chars().count();
            for divider in 2..id_len + 1 {
                if id
                    .chars()
                    .collect::<Vec<char>>()
                    .chunks(id_len / divider)
                    .all_equal()
                {
                    return true;
                }
            }
            false
        }
    }
}

fn parse_range(range: &str, part: &DayPart) -> Result<Vec<u64>, Day2Error> {
    let mut invalid_ids = Vec::<u64>::new();

    let mut range_it = range.split('-');
    let range_begin = range_it
        .next()
        .ok_or(Day2Error::IllformedRange)?
        .parse::<u64>()?;
    let range_end = range_it
        .next()
        .ok_or(Day2Error::IllformedRange)?
        .parse::<u64>()?;

    for id in range_begin..range_end + 1 {
        if is_id_invalid(&(id.to_string()), part) {
            invalid_ids.push(id);
        }
    }

    Ok(invalid_ids)
}

fn parse_file(input: &str, part: DayPart) -> Result<u64, Day2Error> {
    let mut accumulator = 0;
    for range in input.split(',') {
        accumulator += parse_range(range, &part)?.iter().sum::<u64>();
    }
    Ok(accumulator)
}

pub fn solve_day2(path: &str, part: DayPart) -> Result<u64, Day2Error> {
    let file_content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(_) => return Err(Day2Error::FileReadingError),
    };
    parse_file(file_content.as_str(), part)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day2_1_example() {
        // Given
        let file_path = "inputs/day2_example.txt";

        // When
        let result = solve_day2(file_path, DayPart::Part1);

        // Then
        assert_eq!(result, Ok(1227775554));
    }

    #[test]
    fn day2_2_example() {
        // Given
        let file_path = "inputs/day2_example.txt";

        // When
        let result = solve_day2(file_path, DayPart::Part2);

        // Then
        assert_eq!(result, Ok(4174379265));
    }
}
