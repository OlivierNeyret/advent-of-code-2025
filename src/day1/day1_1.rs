use crate::day1::common::{Day1Error, RotationDirection};

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

pub fn parse_file(input: &str) -> Result<u32, Day1Error> {
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
