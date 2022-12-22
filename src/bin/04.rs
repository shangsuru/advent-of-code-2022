use std::ops::RangeInclusive;

use itertools::Itertools;

trait RangeExtension {
    fn contains_range(&self, other: &Self) -> bool;
    fn overlaps_with(&self, other: &Self) -> bool;
}

impl<T: PartialOrd> RangeExtension for RangeInclusive<T> {
    fn contains_range(&self, other: &Self) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }

    fn overlaps_with(&self, other: &Self) -> bool {
        self.contains(other.start()) || self.contains(other.end()) || other.contains_range(self)
    }
}

fn ranges(input: &str) -> impl Iterator<Item = (RangeInclusive<u32>, RangeInclusive<u32>)> + '_ {
    input.lines().map(|line| {
        line.split(',')
            .map(|range| {
                range
                    .split('-')
                    .map(|number| {
                        number
                            .parse::<u32>()
                            .expect("Ranges must be two u32 numbers separated by a hyphen")
                    })
                    .collect_tuple::<(u32, u32)>()
                    .map(|(start, end)| start..=end)
                    .expect("Each range must have a start and an end")
            })
            .collect_tuple::<(RangeInclusive<u32>, RangeInclusive<u32>)>()
            .expect("Each line must have two ranges separated by a comma")
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = ranges(input)
        .map(|(range1, range2)| {
            (range1.contains_range(&range2) || range2.contains_range(&range1)) as u32
        })
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = ranges(input)
        .map(|(range1, range2)| range1.overlaps_with(&range2) as u32)
        .sum();
    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}
