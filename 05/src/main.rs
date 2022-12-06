use nom::bytes::complete::tag;
use nom::character::complete::u32;
use nom::character::complete::{char, digit1};
use nom::combinator::{map, map_res};
use nom::sequence::{pair, preceded, separated_pair};
use nom::IResult;

use std::collections::VecDeque;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl Move {
    fn execute(&self, stack: &mut VecDeque<VecDeque<char>>) {
        let drained: VecDeque<char> = stack[self.from].drain(0..self.count).rev().collect();
        for item in drained.iter() {
            stack[self.to].push_front(*item);
        }
    }
}

fn main() {
    let mut stacks: VecDeque<VecDeque<char>> = VecDeque::new();
    stacks.push_back(vec!['N', 'H', 'S', 'J', 'F', 'W', 'T', 'D'].into());
    stacks.push_back(vec!['G', 'B', 'N', 'T', 'Q', 'P', 'R', 'H'].into());
    stacks.push_back(vec!['V', 'Q', 'L'].into());
    stacks.push_back(vec!['Q', 'R', 'W', 'S', 'B', 'N'].into());
    stacks.push_back("BMVTFDN".chars().collect());
    stacks.push_back("RTHVBDM".chars().collect());
    stacks.push_back("JQBD".chars().collect());
    stacks.push_back("QHZRVJND".chars().collect());
    stacks.push_back("SMHNB".chars().collect());

    let moves: Vec<Move> = aoc_auto::input("5")
        .unwrap()
        .lines()
        .skip_while(|s| !s.starts_with("move"))
        .map(|s| move_parser(s).unwrap().1)
        .collect();
    for m in moves {
        m.execute(&mut stacks);
    }
    println!("");
    for s in stacks {
        print!("{}", s[0]);
    }
    println!("");
}

#[test]
fn move_tester() {
    let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
    let mut stacks: VecDeque<VecDeque<char>> = VecDeque::new();
    stacks.push_back(vec!['N', 'Z'].into());
    stacks.push_back(vec!['D', 'C', 'M'].into());
    stacks.push_back(vec!['P'].into());
    let mut expected: VecDeque<VecDeque<char>> = VecDeque::new();
    expected.push_back(vec!['C'].into());
    expected.push_back(vec!['M'].into());
    expected.push_back(vec!['Z', 'N', 'D', 'P'].into());

    let moves: Vec<Move> = input
        .lines()
        .skip(5)
        .map(|s| move_parser(s).unwrap().1)
        .collect();
    for m in moves {
        println!("{:?}", m);
        m.execute(&mut stacks);
    }
    assert_eq!(stacks, expected);
}

fn move_parser(s: &str) -> IResult<&str, Move> {
    map(
        preceded(tag("move "), pair(u32, from_to_parser)),
        |(count, (from, to))| Move {
            count: count as usize,
            from: from as usize - 1,
            to: to as usize - 1,
        },
    )(s)
}

fn from_to_parser(s: &str) -> IResult<&str, (u32, u32)> {
    preceded(tag(" from "), separated_pair(u32, tag(" to "), u32))(s)
}

#[test]
fn parse_test() {
    let input = "move 3 from 1 to 2
move 1 from 7 to 1
move 1 from 6 to 5";
    let moves: Vec<Move> = input.lines().map(|s| move_parser(s).unwrap().1).collect();
    let comp: Vec<Move> = vec![
        Move {
            count: 3,
            from: 1,
            to: 2,
        },
        Move {
            count: 1,
            from: 7,
            to: 1,
        },
        Move {
            count: 1,
            from: 6,
            to: 5,
        },
    ];
    assert_eq!(moves, comp);
}
