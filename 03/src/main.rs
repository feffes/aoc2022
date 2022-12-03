use std::collections::HashMap;
// space at the start so "a"'s index is 1 cuz lazy
static ALPHABET: &str = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    let input = aoc_auto::input("3").unwrap();

    // part 1
    // let boths: Vec<char> = input.lines().map(|s| both(&s).unwrap()).collect();
    // println!("{}", boths.iter().fold(0, |prev, c| prev + alpha_val(*c)));

    //part 2
    let mut outer: Vec<Vec<&str>> = vec![];
    let mut temp: Vec<&str> = vec![];
    let mut counter = 1;
    for line in input.lines() {
        temp.push(line);
        if counter == 3 {
            outer.push(temp.clone());
            temp = vec![];
            counter = 1;
        } else {
            counter += 1;
        }
    }
    println!(
        "{}",
        outer.iter().fold(0, |prev, s_list| prev
            + alpha_val(char_in_all(s_list.to_vec()).unwrap()))
    );

    //let boths = both(&input);
    // line: [abcd],  len: 4, id: 2, len2??
    // line:
}

fn alpha_val(c: char) -> usize {
    ALPHABET.find(c).unwrap()
}

fn char_in_all(s_list: Vec<&str>) -> Option<char> {
    let mut the_map: HashMap<char, (bool, bool, bool)> = HashMap::new();
    if let [first, second, third] = s_list.as_slice() {
        for c in first.chars() {
            *the_map.entry(c).or_insert((true, false, false));
        }
        for c in second.chars() {
            match the_map.get_mut(&c) {
                Some((_, b2, _)) => {
                    *b2 = true;
                }
                None => {}
            };
        }
        for c in third.chars() {
            match the_map.get_mut(&c) {
                Some((_, _, b3)) => {
                    *b3 = true;
                }
                None => {}
            };
        }
    };

    for (c, v) in the_map.iter() {
        if *v == (true, true, true) {
            return Some(*c);
        }
    }

    None
}

fn both(input: &str) -> Option<char> {
    let (left, right) = input.split_at(input.len() / 2);
    let right_chars: Vec<char> = right.chars().collect();

    let Some(index) = left.find(right_chars.as_slice()) else {
        return None;
    };
    Some(left.chars().collect::<Vec<char>>()[index])
}

#[test]
fn both_test() {
    let test_input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
    let boths: Vec<char> = test_input.lines().map(|s| both(&s).unwrap()).collect();
    let comp: Vec<char> = vec!['p', 'L', 'P', 'v', 't', 's'];
    assert_eq!(boths, comp);
}

#[test]
fn in_all_test() {
    let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
    let mut outer: Vec<Vec<&str>> = vec![];
    let mut temp: Vec<&str> = vec![];
    let mut counter = 1;
    for line in input.lines() {
        temp.push(line);
        if counter == 3 {
            outer.push(temp.clone());
            temp = vec![];
            counter = 1;
        } else {
            counter += 1;
        }
    }
    if let [first, second] = outer
        .iter()
        .map(|s_list| char_in_all(s_list.clone()).unwrap())
        .collect::<Vec<char>>()
        .as_slice()
    {
        println!("{}, {}", first, second);
        assert!(first == &'r' && second == &'Z');
    } else {
        panic!();
    }
}

#[test]
fn alpha_test() {
    assert_eq!(alpha_val('a'), 1);
}
