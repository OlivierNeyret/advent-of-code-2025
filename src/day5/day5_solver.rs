use super::DayPart;

use std::{fs, num::ParseIntError};

#[derive(PartialEq, Debug)]
pub enum Day5Error {
    FileReadingError,
    IllformedRange,
    IllformedId(ParseIntError),
}

impl From<ParseIntError> for Day5Error {
    fn from(err: ParseIntError) -> Self {
        Day5Error::IllformedId(err)
    }
}

struct Range {
    begin: u64,
    end: u64,
}

impl Range {
    fn is_in_range(&self, id: u64) -> bool {
        id >= self.begin && id <= self.end
    }
}

struct FreshIds {
    ids: Vec<Range>,
}

impl FreshIds {
    fn is_fresh(&self, id: u64) -> bool {
        self.ids.iter().any(|range| range.is_in_range(id))
    }
}

struct AvailableIds {
    ids: Vec<u64>,
}

fn parse_file(content: &str) -> Result<(FreshIds, AvailableIds), Day5Error> {
    let mut is_parsing_fresh_ids = true;
    let mut fresh_ids = vec![];
    let mut available_ids = vec![];
    for line in content.lines() {
        if is_parsing_fresh_ids {
            if line.is_empty() {
                is_parsing_fresh_ids = false;
                continue;
            }
            let mut range_it = line.split('-');
            let range_begin = range_it
                .next()
                .ok_or(Day5Error::IllformedRange)?
                .parse::<u64>()?;
            let range_end = range_it
                .next()
                .ok_or(Day5Error::IllformedRange)?
                .parse::<u64>()?;
            fresh_ids.push(Range {
                begin: range_begin,
                end: range_end,
            });
        } else {
            available_ids.push(line.parse()?);
        }
    }
    Ok((
        FreshIds { ids: fresh_ids },
        AvailableIds { ids: available_ids },
    ))
}

pub fn solve_day5(path: &str, _part: DayPart) -> Result<usize, Day5Error> {
    let file_content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(_) => return Err(Day5Error::FileReadingError),
    };
    let (fresh_ids, available_ids) = parse_file(file_content.as_str())?;
    Ok(available_ids
        .ids
        .into_iter()
        .filter(|id| fresh_ids.is_fresh(*id))
        .count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day5_1_example() {
        // Given
        let file_path = "inputs/day5_example.txt";

        // When
        let result = solve_day5(file_path, DayPart::Part1);

        // Then
        assert_eq!(result, Ok(3));
    }
}
