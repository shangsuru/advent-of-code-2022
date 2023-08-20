use itertools::Itertools;

fn find_signal(input: &str, window_size: usize) -> Option<u32> {
    input
    .as_bytes()
    .windows(window_size)
    .position(|window| window.iter().unique().count() == window_size)
    .map(|pos| (pos + window_size) as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
  find_signal(input, 4)
}

pub fn part_two(input: &str) -> Option<u32> {
    find_signal(input, 14)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let mut input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(part_one(input), Some(5));
        
        input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(part_one(input), Some(6));

        input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(part_one(input), Some(10));

        input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(part_one(input), Some(11));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), None);
    }
}
