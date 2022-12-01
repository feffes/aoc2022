use std::cmp::Ord;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct BestPile<T: Ord> {
    pile: BinaryHeap<T>,
    pile_size: usize,
}

impl<T: Ord> BestPile<T> {
    fn new(pile_size: usize) -> BestPile<T> {
        BestPile {
            pile: BinaryHeap::new(),
            pile_size,
        }
    }

    fn push(&mut self, item: T) {
        if self.pile.len() >= self.pile_size {
            if let Some(highest) = self.pile.peek() {
                if highest > &item {
                    self.pile.pop();
                    self.pile.push(item);
                }
            }
        } else {
            self.pile.push(item);
        }
    }
}

#[derive(Clone, Debug, Eq)]
struct Elf {
    food: Vec<u64>,
}

impl Elf {
    fn total(&self) -> u64 {
        self.food.iter().fold(0, |prev, val| prev + val)
    }
    fn push(&mut self, val: u64) {
        self.food.push(val);
    }
}

impl Default for Elf {
    fn default() -> Self {
        Self { food: vec![] }
    }
}

impl Ord for Elf {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.total().cmp(&other.total())
    }
}

impl PartialOrd for Elf {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Elf {
    fn eq(&self, other: &Self) -> bool {
        self.total() == other.total()
    }
}

fn main() {
    let input = std::fs::read_to_string("input").expect("Couldn't read input file");
    let mut best_elves: BestPile<Reverse<Elf>> = BestPile::new(3);
    let mut elf = Elf::default();
    for line in input.lines() {
        if line.is_empty() {
            best_elves.push(Reverse(elf.clone()));
            elf = Elf::default();
        } else if let Ok(num) = line.parse::<u64>() {
            elf.push(num);
        }
    }
    println!(
        "{:?}",
        best_elves
            .pile
            .iter()
            .fold(0, |prev, val| prev + val.0.total())
    );
}

#[test]
fn total_test() {
    let elf = Elf {
        food: vec![1, 2, 3, 4],
    };

    assert_eq!(10, elf.total());
}
