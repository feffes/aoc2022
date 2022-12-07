use nom::branch::alt;
use nom::bytes::complete::{is_a, tag, take_till, take_until, take_while};
use nom::character::complete::{char, digit1, newline, one_of};
use nom::character::complete::{line_ending, u64};
use nom::character::is_newline;
use nom::combinator::{eof, map, map_res, not};
use nom::multi::{many1, separated_list0};
use nom::sequence::{delimited, pair, preceded, separated_pair};
use nom::IResult;

use itertools::Itertools;

use std::cell::RefCell;
use std::collections::HashMap;
use std::mem::replace;
use std::rc::Rc;

type Thing = Rc<RefCell<INode>>;
#[derive(Debug, Clone)]
enum INode {
    Dir {
        val: Vec<Thing>,
        //we don't know the name during parsing
        name: String,
    },
    // contains the size
    File {
        val: usize,
        name: String,
    },
}

impl INode {
    // finds the inode by name
    fn find(&self, s: &str) -> Option<Thing> {
        use INode::Dir;
        // outer dir
        if let Dir { val, .. } = self {
            for temp in val.into_iter() {
                // for every dir in the outer dir
                if let Dir { name, .. } = &*temp.borrow() {
                    if s == name {
                        return Some(temp.clone());
                    }
                }
            }
        };

        None
    }

    fn sum(&self) -> usize {
        use INode::*;
        match self {
            Dir { val, .. } => val
                .iter()
                .map(|c| c.to_owned().borrow().sum())
                .sum::<usize>(),
            File { val, .. } => *val,
        }
    }

    // fn empty(&self) -> Option<Rc<Self>> {

    //     if let Dir { val, ..} = self {

    //     }
    // }
}

#[derive(Debug, Clone)]
enum Command {
    CCD(CD),
    LS(Vec<Thing>),
}

#[derive(Debug, Clone)]
enum CD {
    Rel(String),
    Back,
    Root,
}

fn main() {
    let mut input = aoc_auto::input("7").unwrap();
    // hacky workaround cuz my parser is buggy, it needs to detect either $ or EOF and i can't figure it out lmao
    input = format!("{}{}", input, "\n$");
    //     let input = "$ cd /
    // $ ls
    // dir a
    // 14848514 b.txt
    // 8504156 c.dat
    // dir d
    // $ cd a
    // $ ls
    // dir e
    // 29116 f
    // 2557 g
    // 62596 h.lst
    // $ cd e
    // $ ls
    // 584 i
    // $ cd ..
    // $ cd ..
    // $ cd d
    // $ ls
    // 4060174 j
    // 8033020 d.log
    // 5626152 d.ext
    // 7214296 k
    // $"
    //     .to_string();

    let (s, cmds) = cmd_parser(input.as_str()).unwrap();
    // We go through each command, keeping track of PWD
    let mut pwd: Vec<String> = vec![];

    dbg!(s);
    // Save all the values in a hashmap, putting the pwd as key, and the updated
    // dir inode as val

    let mut the_map: HashMap<Vec<String>, Thing> = HashMap::new();

    // pwd is empty first cuz idk how to deal with root ayy lmao
    // also we spawn

    for cmd in cmds {
        use Command::*;
        use INode::Dir;
        match cmd {
            LS(inodes) => {
                the_map.insert(
                    pwd.clone(),
                    Rc::new(RefCell::new(Dir {
                        val: inodes,
                        name: pwd.get(pwd.len() - 1).unwrap_or(&"/".into()).clone(),
                    })),
                );
            }
            CCD(CD::Back) => {
                pwd.pop();
            }
            CCD(CD::Root) => {
                pwd = vec![];
            }
            CCD(CD::Rel(r)) => {
                pwd.push(r);
            }
        }
    }

    for (k, v) in the_map
        .iter()
        .sorted_by(|(k1, _), (k2, _)| Ord::cmp(&k2.len(), &k1.len()))
    {
        if k.len() != 0 {
            let mut temp = k.clone();
            let name = temp.pop().unwrap();
            //rc clone
            let rclone = &the_map[&temp];
            // get the thing refcell from the outer thing's list, this should be empty
            // at this point
            rclone
                .borrow_mut()
                .find(&name)
                .unwrap()
                .replace(v.borrow().clone());
            // thing.replace(dbg!(v.borrow().clone()));
        }
    }

    // part 1
    let mut lessthan: Vec<usize> = vec![];

    for (k, v) in the_map.iter() {
        let temp: usize = v.clone().borrow().sum();
        if temp <= 100_000 {
            // use INode::Dir;
            // let new_inode = if let Dir { name, .. } = &*v.borrow() {
            //     Dir {
            //         name: name.clone(),
            //         val: vec![],
            //     }
            // } else {
            //     panic!()
            // };
            // v.replace(new_inode);
            lessthan.push(temp);
            // println!("deleted {:?}", k);
        }
    }
    // let deleted = lessthan.iter().sum::<usize>();

    // println!("deleted {}", deleted);
    // part 2
    let mut sums: Vec<usize> = vec![];

    let how_much = the_map[&vec![]].borrow().sum();

    dbg!(how_much);

    for (k, v) in the_map.iter() {
        let temp: usize = v.to_owned().borrow().sum();
        if temp >= 50_000_000 - how_much {
            dbg!(k);
            sums.push(temp);
        }
    }
    sums.sort();
    dbg!(sums);
}

fn cmd_parser(s: &str) -> IResult<&str, Vec<Command>> {
    many1(alt((cd_parser, ls_parser)))(s)
}

fn cd_inner_parser(s: &str) -> IResult<&str, CD> {
    use CD::*;
    let (s, inner) = take_until("\n")(s)?;
    let cd = match inner {
        "/" => Root,
        ".." => Back,
        val => Rel(val.into()),
    };
    Ok((s, cd))
}

fn cd_parser(s: &str) -> IResult<&str, Command> {
    let (s, cd) = delimited(tag("$ cd "), cd_inner_parser, newline)(s)?;
    Ok((s, Command::CCD(cd)))
}

fn ls_parser(s: &str) -> IResult<&str, Command> {
    use Command::LS;
    let (s, _) = tag("$ ls\n")(s)?;
    let (s, thing) = take_until("$")(s)?;
    let (_, output) = ls_output_parser(thing)?;

    Ok((s, LS(output)))
}

fn ls_output_parser(s: &str) -> IResult<&str, Vec<Thing>> {
    let (s, v) = separated_list0(newline, ls_output_line_parser)(s)?;
    Ok((s, v))
}

fn ls_output_line_parser(s: &str) -> IResult<&str, Thing> {
    let (s, temp) = alt((ls_output_file_parser, ls_output_dir_parser))(s)?;
    Ok((s, Rc::new(RefCell::new(temp))))
}

fn ls_output_dir_parser(s: &str) -> IResult<&str, INode> {
    use INode::Dir;
    let (s, rest) = preceded(tag("dir "), take_until("\n"))(s)?;
    let out = Dir {
        name: rest.into(),
        val: vec![],
    };
    Ok((s, out))
}
fn ls_output_file_parser(s: &str) -> IResult<&str, INode> {
    use INode::File;
    let (s, (val, name)) = separated_pair(u64, char(' '), is_a("qwertyuiopasdfghjklzxcvbnm."))(s)?;
    let out = File {
        name: name.into(),
        val: val as usize,
    };
    Ok((s, out))
}
