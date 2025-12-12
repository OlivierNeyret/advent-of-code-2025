use itertools::Itertools;

use super::DayPart;
use std::{fs, num::ParseIntError};

#[derive(PartialEq, Debug)]
pub enum Day3Error {
    FileReadingError,
    IllformedBank(ParseIntError),
    EmptyBank,
}

impl From<ParseIntError> for Day3Error {
    fn from(err: ParseIntError) -> Self {
        Day3Error::IllformedBank(err)
    }
}

fn parse_bank(bank: &str, _part: &DayPart) -> Result<Vec<u64>, Day3Error> {
    bank.chars()
        .collect::<Vec<char>>()
        .into_iter()
        .chunks(1)
        .into_iter()
        .map(|chunk| {
            let chunk_str: String = chunk.collect();
            chunk_str.parse::<u64>().map_err(Day3Error::IllformedBank)
        })
        .collect()
}

fn max_jolts_from_bank(bank: Vec<u64>, _part: &DayPart) -> Result<u64, Day3Error> {
    let mut first_max = 0;
    let mut second_max = 0;
    let nb_batteries = bank.len();
    for (idx, jolt) in bank.into_iter().enumerate() {
        if jolt > second_max {
            second_max = jolt;
            if second_max > first_max && idx < nb_batteries - 1 {
                first_max = second_max;
                second_max = 0;
            }
        }
    }

    Ok(first_max * 10 + second_max)
}

fn calculate_bank_jolts(bank: &str, _part: &DayPart) -> Result<u64, Day3Error> {
    max_jolts_from_bank(parse_bank(bank, _part)?, _part)
}

fn parse_file(input: &str, part: DayPart) -> Result<u64, Day3Error> {
    input
        .lines()
        .map(|bank| calculate_bank_jolts(bank, &part))
        .sum()
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
        let file_path = "inputs/day3.txt";
        // let file_path = "inputs/day3_example.txt";

        // When
        let result = solve_day3(file_path, DayPart::Part1);

        // Then
        assert_eq!(result, Ok(357));
    }
}
