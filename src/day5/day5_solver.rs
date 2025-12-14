use super::DayPart;

use std::{fs, num::ParseIntError, vec};

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

#[derive(Clone, Copy)]
struct Range {
    begin: usize,
    end: usize,
}

impl Range {
    fn contains(&self, id: usize) -> bool {
        id >= self.begin && id <= self.end
    }

    fn is_overlapping(&self, other: &Range) -> bool {
        self.begin <= other.end && other.begin <= self.end
    }

    fn union(&self, other: &Range) -> Range {
        Range {
            begin: self.begin.min(other.begin),
            end: self.end.max(other.end),
        }
    }
}

struct FreshIds {
    ids: Vec<Range>,
}

impl FreshIds {
    fn new(mut ranges: Vec<Range>) -> FreshIds {
        ranges.sort_by(|a, b| a.begin.cmp(&b.begin));

        let mut i = 0;
        while i < ranges.len() - 1 {
            if ranges[i].is_overlapping(&ranges[i + 1]) {
                ranges[i] = ranges[i].union(&ranges[i + 1]);
                ranges.remove(i + 1);
            } else {
                i += 1;
            }
        }

        FreshIds { ids: ranges }
        // if ids.is_empty() {
        //     return FreshIds { ids: vec![] }
        // }

        // let mut squashed_ranges = vec![];
        // squashed_ranges.push(ids[0]);

        // for range in ids.iter().skip(1) {
        //     // let mut range_container: Option<&Range> = None;
        //     for squashed_range in squashed_ranges.iter_mut() {
        //         if range.is_overlapping(squashed_range) {
        //             *squashed_range = Range {
        //                 begin: range.begin.min(squashed_range.begin),
        //                 end: range.end.max(squashed_range.end),
        //             };
        //         }
        //     }
        // }

        // FreshIds { ids:squashed_ranges }
    }

    fn is_fresh(&self, id: usize) -> bool {
        self.ids.iter().any(|range| range.contains(id))
    }

    fn count(&self) -> usize {
        self.ids
            .iter()
            .fold(0, |c, range| c + range.end - range.begin + 1)
    }
}

struct AvailableIds {
    ids: Vec<usize>,
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
                .parse::<usize>()?;
            let range_end = range_it
                .next()
                .ok_or(Day5Error::IllformedRange)?
                .parse::<usize>()?;
            fresh_ids.push(Range {
                begin: range_begin,
                end: range_end,
            });
        } else {
            available_ids.push(line.parse()?);
        }
    }
    Ok((
        FreshIds::new(fresh_ids),
        AvailableIds { ids: available_ids },
    ))
}

pub fn solve_day5(path: &str, part: DayPart) -> Result<usize, Day5Error> {
    let file_content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(_) => return Err(Day5Error::FileReadingError),
    };
    let (fresh_ids, available_ids) = parse_file(file_content.as_str())?;
    match part {
        DayPart::Part1 => Ok(available_ids
            .ids
            .into_iter()
            .filter(|id| fresh_ids.is_fresh(*id))
            .count()),
        DayPart::Part2 => Ok(fresh_ids.count()),
    }
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

    #[test]
    fn day5_2_example() {
        // Given
        let file_path = "inputs/day5_example.txt";

        // When
        let result = solve_day5(file_path, DayPart::Part2);

        // Then
        assert_eq!(result, Ok(14));
    }
}
