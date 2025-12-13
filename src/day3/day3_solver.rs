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

fn parse_bank(bank: &str) -> Result<Vec<u64>, Day3Error> {
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

fn convert_to_jolts(batteries: Vec<u64>) -> u64 {
    let mut result: u64 = 0;
    for (idx, value) in batteries.into_iter().rev().enumerate() {
        result += value * (10_u64.pow(idx as u32));
    }
    result
}

fn max_jolts_from_bank(bank: Vec<u64>, part: &DayPart) -> Result<u64, Day3Error> {
    let nb_batteries_to_pick = match part {
        DayPart::Part1 => 2,
        DayPart::Part2 => 12,
    };

    let mut selected_batteries = Vec::new();
    let nb_batteries = bank.len();
    let mut start_idx: usize = 0;

    for i in 0..nb_batteries_to_pick {
        let mut current_max: u64 = 0;
        let mut current_max_idx: usize = 0;
        for (idx, jolt) in bank
            .iter()
            .skip(start_idx)
            .take((nb_batteries - start_idx) - (nb_batteries_to_pick - i - 1))
            .enumerate()
        {
            if *jolt > current_max {
                current_max = *jolt;
                current_max_idx = idx + start_idx;
            }
        }
        selected_batteries.push(current_max);
        start_idx = current_max_idx + 1;
    }

    Ok(convert_to_jolts(selected_batteries))
}

fn calculate_bank_jolts(bank: &str, part: &DayPart) -> Result<u64, Day3Error> {
    max_jolts_from_bank(parse_bank(bank)?, part)
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

    #[test]
    fn day3_2_example() {
        // Given
        let file_path = "inputs/day3.txt";
        // let file_path = "inputs/day3_example.txt";

        // When
        let result = solve_day3(file_path, DayPart::Part2);

        // Then
        assert_eq!(result, Ok(3121910778619));
    }
}
