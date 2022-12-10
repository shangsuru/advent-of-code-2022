use std::collections::HashSet;

struct Rucksack<'a> {
    first_compartment: &'a str,
    second_compartment: &'a str,
}

struct Group<'a> {
    elf1: Rucksack<'a>,
    elf2: Rucksack<'a>,
    elf3: Rucksack<'a>,
}

impl<'a> Rucksack<'a> {
    fn new(input: &str) -> Rucksack {
        let (first_compartment, second_compartment) = input.split_at(input.len() / 2);
        Rucksack {
            first_compartment,
            second_compartment,
        }
    }

    fn common_item(&self) -> char {
        let mut set1 = HashSet::new();
        for c in self.first_compartment.chars() {
            set1.insert(c);
        }
        let mut set2 = HashSet::new();
        for c in self.second_compartment.chars() {
            set2.insert(c);
        }

        *set1.intersection(&set2).into_iter().next().unwrap()
    }

    fn all_items(&self) -> HashSet<char> {
        let mut set = HashSet::new();
        for c in self.first_compartment.chars() {
            set.insert(c);
        }
        for c in self.second_compartment.chars() {
            set.insert(c);
        }
        set
    }
}

impl<'a> Group<'a> {
    fn new(input: [&str; 3]) -> Group {
        Group {
            elf1: Rucksack::new(input[0]),
            elf2: Rucksack::new(input[1]),
            elf3: Rucksack::new(input[2]),
        }
    }

    fn badge(&self) -> char {
        let intersection12: HashSet<char> = self
            .elf1
            .all_items()
            .intersection(&self.elf2.all_items())
            .cloned()
            .collect();
        let set3 = self.elf3.all_items();
        let intersection = intersection12.intersection(&set3);
        *intersection.into_iter().next().unwrap()
    }
}

fn get_priority(item: char) -> u32 {
    if item.is_lowercase() {
        (item as u32 - 'a' as u32) + 1
    } else if item.is_uppercase() {
        (item as u32 - 'A' as u32) + 27
    } else {
        0
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let item = Rucksack::new(line).common_item();
                get_priority(item)
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum: u32 = 0;
    let mut it = input.lines();

    loop {
        let lines: Vec<&str> = it.by_ref().take(3).collect();
        if lines.len() < 3 {
            break Some(sum);
        }

        let badge = Group::new(lines.try_into().unwrap()).badge();
        sum += get_priority(badge);
    }
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
        assert_eq!(part_one(&input), Some(12));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(11));
    }
}
