use std::{fs, num::ParseIntError};

#[derive(PartialEq, Debug)]
pub enum Day2Error {
    FileReadingError,
    IllformedId(ParseIntError),
}

fn is_id_invalid(id: &str) -> bool {
    let (left, right) = id.split_at(id.len() / 2);
    left == right
}

fn parse_range(range: &str) -> Result<Vec<i32>, Day2Error> {
    Ok(range
        .split('-')
        //TODO: fix logic, it should iterate from left to right and for each of them call is_id_valid
        .filter(|id| is_id_invalid(&id))
        .map(|s| s.parse::<i32>().map_err(Day2Error::IllformedId))        
        .collect::<Result<Vec<i32>,_>>()?
        .into_iter()
        .collect())
}

fn parse_file(input: &str) -> Result<i32, Day2Error> {
    let mut accumulator = 0;
    for range in input.split(',') {
        accumulator += parse_range(range)?.iter().sum::<i32>(); 
    }
    Ok(accumulator)
}

pub fn solve_day2_1(path: &str) -> Result<i32, Day2Error> {
    let file_content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(_) => return Err(Day2Error::FileReadingError),
    };
    parse_file(file_content.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day2_1_example() {
        // Given
        let file_path = "inputs/day2_example.txt";

        // When
        let result = solve_day2_1(file_path);

        // Then
        assert_eq!(result, Ok(1227775554));
    }
}
