use itertools::Itertools;

use super::DayPart;
use std::{fs, num::ParseIntError};

#[derive(PartialEq, Debug)]
pub enum Day3Error {
    FileReadingError,
    IllformedBank(ParseIntError),
}

impl From<ParseIntError> for Day3Error {
    fn from(err: ParseIntError) -> Self {
        Day3Error::IllformedBank(err)
    }
}

fn parse_bank(bank: &str, _part: &DayPart) -> Result<u64, Day3Error> {
    //TODO: isolate this in a function parse_bank and rename this function calculate_bank_jolts
    let all_jolts = bank.chars()
        .collect::<Vec<char>>()
        .into_iter()
        .chunks(1)
        .into_iter()
        .map(|chunk| {
            let chunk_str: String = chunk.collect();
            chunk_str
                .parse::<u64>()
                .map_err(|e| Day3Error::IllformedBank(e))
        })
        .collect();

    //TODO: iterate over all_jolts and find the first max value.
    // Then find the next max value, starting from previous max
}

fn parse_file(input: &str, part: DayPart) -> Result<u64, Day3Error> {
    input.lines().map(|bank| parse_bank(bank, &part)).sum()
}

pub fn solve_day3(path: &str, part: DayPart) -> Result<u64, Day3Error> {
    let file_content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(_) => return Err(Day3Error::FileReadingError),
    };
    parse_file(file_content.as_str(), part)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day3_1_example() {
        // Given
        let file_path = "inputs/day3_example.txt";

        // When
        let result = solve_day3(file_path, DayPart::Part1);

        // Then
        assert_eq!(result, Ok(357));
    }
}
