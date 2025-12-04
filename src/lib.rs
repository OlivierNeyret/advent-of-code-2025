mod day1;

use day1::common::Day1Error;
use day1::{day1_1, day1_2};
use std::fs;

pub fn solve_day1_1(path: &str) -> Result<u32, Day1Error> {
    let file_content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(_) => return Err(Day1Error::FileReadingError),
    };
    day1_1::parse_file(file_content.as_str())
}

pub fn solve_day1_2(path: &str) -> Result<u32, Day1Error> {
    let file_content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(_) => return Err(Day1Error::FileReadingError),
    };
    day1_2::parse_file(file_content.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_1_example() {
        // Given
        let file_path = "inputs/day1_example.txt";

        // When
        let result = solve_day1_1(file_path);

        // Then
        assert_eq!(result, Ok(3));
    }

    #[test]
    fn day1_2_example() {
        // Given
        let file_path = "inputs/day1_example.txt";

        // When
        let result = solve_day1_2(file_path);

        // Then
        assert_eq!(result, Ok(6));
    }
}
