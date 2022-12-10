use std::cmp::max;

pub fn part_one(input: &str) -> Option<u32> {
    let mut max_calories: u32 = 0;
    let mut elf_calories: u32 = 0;
    for line in input.lines() {
        if line.is_empty() {
            max_calories = max(max_calories, elf_calories);
            elf_calories = 0;
        } else {
            elf_calories += line.parse::<u32>().unwrap();
        }
    }
    Some(max_calories)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut all_elves_calories: Vec<u32> = vec![];
    let mut current_elf_calories: Vec<u32> = vec![];

    for line in input.lines() {
        if line.is_empty() {
            all_elves_calories.push(current_elf_calories.iter().sum());
            current_elf_calories = vec![];
        } else {
            current_elf_calories.push(line.parse::<u32>().unwrap());
        }
    }

    all_elves_calories.sort_by_key(|&v| std::cmp::Reverse(v));
    Some(all_elves_calories.iter().take(3).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(10));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(26));
    }
}
