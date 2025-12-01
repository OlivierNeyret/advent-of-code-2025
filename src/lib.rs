use std::{fs, num::ParseIntError};

#[derive(PartialEq, Debug)]
pub enum Day1Error {
    IllformedOffset(ParseIntError),
    IllformedDirection,
    FileReadingError,
}

enum RotationDirection {
    Left,
    Right,
}

fn rotate(current: i32, direction: RotationDirection, offset: i32) -> i32 {
    let mut new_position = current;
    match direction {
        RotationDirection::Left => {
            new_position -= offset;
        }
        RotationDirection::Right => {
            new_position += offset;
        }
    }
    new_position.rem_euclid(100)
}

fn parse_line(line: &str) -> Result<(RotationDirection, i32), Day1Error> {
    let (direction_str, offset_str) = line.split_at(1);
    let offset = match offset_str.parse::<i32>() {
        Ok(value) => value,
        Err(e) => return Err(Day1Error::IllformedOffset(e)),
    };

    let direction = match direction_str {
        "L" => RotationDirection::Left,
        "R" => RotationDirection::Right,
        _ => {return Err(Day1Error::IllformedDirection) },
    };
    Ok((direction, offset))
}

fn parse_file(input: &str) -> Result<u32, Day1Error> {
    let mut count_zero: u32 = 0;
    let mut current_pointed_value: i32 = 50;
    for line in input.lines() {
        let (direction, offset) = parse_line(line)?;

        current_pointed_value = rotate(current_pointed_value, direction, offset);

        if current_pointed_value == 0 {
            count_zero += 1;
        }
    }
    Ok(count_zero)
}

pub fn day1_file(path: &str) ->  Result<u32, Day1Error> {
    let file_content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(_) => { return Err(Day1Error::FileReadingError) },
    };
    parse_file(file_content.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_example() {
        // Given
        let file_path = "inputs/day1_example.txt";

        // When
        let result = day1_file(file_path);

        // Then
        println!("{:?}", result);
        assert_eq!(result, Ok(3));
    }
}
