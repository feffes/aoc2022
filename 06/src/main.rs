use itertools::Itertools;
// use nom::bytes::complete::tag;
// use nom::character::complete::u32;
// use nom::character::complete::{char, digit1};
// use nom::combinator::{map, map_res};
// use nom::sequence::{pair, preceded, separated_pair};
// use nom::IResult;

fn main() {
    let input = aoc_auto::input("6").unwrap();
    // first window = "abcd", enumerate 0, second is bcde, enumerate 1, 4+i
    for (i, w) in input.chars().collect::<Vec<char>>().windows(14).enumerate() {
        println!("processing {:?}", w);
        // if w.iter().duplicates().count() == 0 {
        //     println!("{}", i + 4);
        //     break;
        // }
        if w.iter().duplicates().count() == 0 {
            println!("{}", i + 14);
            break;
        }
    }
}
