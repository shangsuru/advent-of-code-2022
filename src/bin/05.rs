use itertools::Itertools;
use std::fmt::Display;

#[derive(Clone, Copy)]
struct Crate(char);

impl Display for Crate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Instruction {
    n: usize,    // how many crates to move
    from: usize, // from which pile to move them
    to: usize,   // to which pile to move them
}

struct Piles {
    stacks: Vec<Vec<Crate>>,
}

impl Piles {
    // Creates new stacks of crates from ASCII description
    fn new(str_repr: &str) -> Self {
        let number_of_stacks = str_repr
            .trim()
            .chars()
            .last()
            .expect("Last character is the number of the last stack")
            .to_digit(10)
            .expect("Last char is a digit") as usize;
        let mut stacks: Vec<Vec<Crate>> = vec![Vec::new(); number_of_stacks];

        // Get the crates of each stack
        for s in 1..=number_of_stacks {
            let pos = str_repr
                .lines()
                .last()
                .expect("Last line contains the number of each stack")
                .find(&s.to_string())
                .unwrap();

            let stack: Vec<Crate> = str_repr.lines().rev().skip(1).map(|l| {
                Crate(l.chars().nth(pos).expect("Above each number at the same position are the chars corresponding to the crates of that stack"))
            }).filter(|c| c.0 != ' ').collect();

            stacks[s - 1] = stack;
        }

        Self { stacks }
    }

    // Moves n crates from one stack to another, as if each crate would be moved
    // one by one by a crane. If new_version is true, the crane picks up all crates
    // at once and moves them, so their order is not reversed
    fn move_crates(&mut self, ins: Instruction, new_version: bool) {
        // Make sure that the indices are valid
        assert!(ins.from < self.stacks.len() && ins.to < self.stacks.len());

        // Take the crates from the "from" stack
        let size_of_from_crate = self.stacks[ins.from].len();
        let crates = self.stacks[ins.from].split_off(size_of_from_crate - ins.n);

        // Append the crates to the "to" stack
        if new_version {
            self.stacks[ins.to].extend(crates.into_iter());
        } else {
            self.stacks[ins.to].extend(crates.into_iter().rev());
        }
    }

    fn top_crates(self) -> String {
        self.stacks.iter().map(|pile| pile.last().unwrap()).join("")
    }
}

// Extracts the numbers from a move instruction of the form
// "Move 5 from 3 to 1". E.g. in this case it will return 5,2,0
// because the indices of the vectors start with 0.
fn parse_move(move_str: &str) -> Instruction {
    let mut iter = move_str.split_whitespace();
    iter.next(); // Skip the first word (move)
    let n = iter.next().unwrap().parse::<usize>().unwrap();
    iter.next(); // Skip the next word (from)
    let from = iter.next().unwrap().parse::<usize>().unwrap();
    iter.next(); // Skip the next word (to)
    let to = iter.next().unwrap().parse::<usize>().unwrap();

    Instruction {
        n,
        from: from - 1,
        to: to - 1,
    } // Index of vectors start at 0
}

// Returns the crates on top of each stack concatenated as a string
fn top_crates_from_moves(input: &str, new_version: bool) -> String {
    let (drawing, moves) = input
        .split_once("\n\n")
        .expect("Drawing of the stacks and moves are separeted by two newlines");
    let mut stacks = Piles::new(drawing);

    moves.trim().lines().for_each(|l| {
        stacks.move_crates(parse_move(l), new_version);
    });

    stacks.top_crates()
}

pub fn part_one(input: &str) -> Option<String> {
    Some(top_crates_from_moves(input, false))
}

pub fn part_two(input: &str) -> Option<String> {
    Some(top_crates_from_moves(input, true))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}
