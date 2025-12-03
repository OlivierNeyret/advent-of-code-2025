use crate::day1::common::{Day1Error, RotationDirection};

struct Rotation {
    pub direction: RotationDirection,
    pub offset: i32,
}

struct RotationResult {
    pub position: i32,
    pub nb_zero_pointed: u32,
}

fn rotate(current: i32, rotation: Rotation) -> RotationResult {
    let mut new_position = current;
    let mut nb_zeros: u32 = 0;
    for _i in 0..rotation.offset {
        new_position += match rotation.direction {
            RotationDirection::Left => -1,
            RotationDirection::Right => 1,
        };
        new_position = new_position.rem_euclid(100);
        if new_position == 0 {
            nb_zeros +=1;
        }
    }
    RotationResult { position: new_position, nb_zero_pointed: nb_zeros }
}

fn parse_line(line: &str) -> Result<Rotation, Day1Error> {
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
    Ok(Rotation { direction, offset })
}

pub fn parse_file(input: &str) -> Result<u32, Day1Error> {
    let mut count_zero: u32 = 0;
    let mut current_pointed_value: i32 = 50;
    for line in input.lines() {
        let rotation = parse_line(line)?;

        let rotation_result = rotate(current_pointed_value, rotation);
        current_pointed_value = rotation_result.position;
        count_zero += rotation_result.nb_zero_pointed;
    }
    Ok(count_zero)
}
