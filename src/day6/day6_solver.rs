use super::DayPart;

use std::{fs, num::ParseIntError, vec};

#[derive(PartialEq, Debug)]
pub enum Day6Error {
    FileReadingError,
    IllformedId(ParseIntError),
}

//TODO: add a dedicated struct for one Problem?

struct Problems {
    all_values: Vec<Vec<u64>,
    //TODO: add something to store the operations
}

fn sum_cols(all_values: Vec<Vec<u64>>) -> Vec<u64> {

}

fn parse_file(content: &str) -> Result<Vec<Vec<u64>>, Day6Error> {

}

pub fn solve_day6(path: &str, _part: DayPart) -> Result<u64, Day6Error> {
    let file_content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(_) => return Err(Day6Error::FileReadingError),
    };
    let all_values = parse_file(file_content.as_str())?;
    Ok(sum_cols(all_values).iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day6_1_example() {
        // Given
        let file_path = "inputs/day6_example.txt";

        // When
        let result = solve_day6(file_path, DayPart::Part1);

        // Then
        assert_eq!(result, Ok(4277556));
    }
}
