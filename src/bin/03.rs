use item::Item;
use itertools::Itertools;
use std::collections::HashSet;

mod item {

    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub(crate) struct Item(u8);

    impl TryFrom<u8> for Item {
        type Error = String;

        fn try_from(value: u8) -> Result<Self, Self::Error> {
            match value {
                b'a'..=b'z' | b'A'..=b'Z' => Ok(Item(value)),
                _ => Err("Expects only alphabetic characters".to_string()),
            }
        }
    }

    impl Item {
        pub(crate) fn priority(self) -> u32 {
            match self {
                Item(b'a'..=b'z') => 1 + (self.0 - b'a') as u32,
                Item(b'A'..=b'Z') => 27 + (self.0 - b'A') as u32,
                _ => unreachable!(),
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let sum: u32 = input
        .lines()
        .map(|line| {
            let (first, second) = line.split_at(line.len() / 2);
            let first_compartment = first
                .bytes()
                .map(Item::try_from)
                .collect::<Result<HashSet<Item>, _>>()
                .unwrap();
            let second_compartment = second
                .bytes()
                .map(Item::try_from)
                .collect::<Result<HashSet<Item>, _>>()
                .unwrap();

            let common_item = first_compartment
                .intersection(&second_compartment)
                .into_iter()
                .next()
                .unwrap();
            common_item.priority()
        })
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let rucksacks = input.lines().map(|line| {
        line.bytes()
            .map(Item::try_from)
            .collect::<Result<HashSet<_>, _>>()
    });

    let sum = itertools::process_results(rucksacks, |rs| {
        rs.tuples()
            .map(|(a, b, c)| {
                a.iter()
                    .copied()
                    .find(|i| b.contains(i) && c.contains(i))
                    .map(|i| i.priority())
                    .unwrap_or_default()
            })
            .sum::<u32>()
    })
    .unwrap();

    Some(sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(38));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(11));
    }
}
