use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1};
use nom::combinator::map_res;
use nom::sequence::separated_pair;
use nom::IResult;

use gcollections::ops::Overlap;
use interval::ops::*;
use interval::Interval;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
struct Section {
    start: u64,
    end: u64,
}

impl Section {
    fn contains(&self, other: &Self) -> bool {
        other.start <= self.start && other.end >= self.end
    }
    fn overlaps(&self, other: &Self) -> bool {
        let left = Interval::new(self.start, self.end);
        let right = Interval::new(other.start, other.end);
        left.overlap(&right)
    }
}

fn main() {
    let input = aoc_auto::input("4").unwrap();
    let section_pairs: Vec<(Section, Section)> =
        input.lines().map(|s| line_parser(s).unwrap().1).collect();
    let mut counter = 0;
    // for (left, right) in section_pairs {
    //     if left.contains(&right) || right.contains(&left) {
    //         counter += 1;
    //     }
    // }

    for (left, right) in section_pairs {
        if left.overlaps(&right) {
            counter += 1;
        }
    }
    println!("{}", counter);
}

fn u64_parser(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse)(input)
}

fn section_parser(s: &str) -> IResult<&str, Section> {
    let (rest, (start, end)) = separated_pair(u64_parser, char('-'), u64_parser)(s)?;
    Ok((rest, Section { start, end }))
}

fn line_parser(s: &str) -> IResult<&str, (Section, Section)> {
    let (rest, (left, right)) = separated_pair(section_parser, char(','), section_parser)(s)?;
    Ok((rest, (left, right)))
}
