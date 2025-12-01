use std::num::ParseIntError;

#[derive(PartialEq, Debug)]
pub enum Day1Error {
    IllformedOffset(ParseIntError),
    IllformedDirection,
    FileReadingError,
}

pub enum RotationDirection {
    Left,
    Right,
}
