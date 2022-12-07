use std::process::exit;

enum GameResult {
    Win,
    Draw,
    Loss,
}

impl GameResult {
    fn to_points(&self) -> u32 {
        match self {
            GameResult::Win => 6,
            GameResult::Draw => 3,
            GameResult::Loss => 0,
        }
    }
}

#[derive(PartialEq)]
enum Choice {
    Rock,
    Paper,
    Scissor,
}

impl Choice {
    fn to_points(&self) -> u32 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissor => 3,
        }
    }

    fn against(self, opponent: Choice) -> GameResult {
        if self == opponent {
            return GameResult::Draw;
        }
        if self == get_winning_choice(opponent) {
            return GameResult::Win;
        }
        GameResult::Loss
    }
}

fn get_winning_choice(opponent: Choice) -> Choice {
    match opponent {
        Choice::Rock => Choice::Paper,
        Choice::Paper => Choice::Scissor,
        Choice::Scissor => Choice::Rock,
    }
}

fn get_losing_choice(opponent: Choice) -> Choice {
    match opponent {
        Choice::Rock => Choice::Scissor,
        Choice::Paper => Choice::Rock,
        Choice::Scissor => Choice::Paper,
    }
}

fn to_choice(input: &str) -> Choice {
    match input {
        "A" | "X" => Choice::Rock,
        "B" | "Y" => Choice::Paper,
        "C" | "Z" => Choice::Scissor,
        _ => exit(1),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut score: u32 = 0;

    for line in input.split('\n') {
        if line.is_empty() {
            continue;
        }

        let mut choices = line.split_whitespace();
        let opponent: Choice = to_choice(choices.next().unwrap());
        let player: Choice = to_choice(choices.next().unwrap());

        score += player.to_points();
        score += player.against(opponent).to_points()
    }

    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut score: u32 = 0;

    for line in input.split('\n') {
        if line.is_empty() {
            continue;
        }

        let mut choices = line.split_whitespace();
        let opponent: Choice = to_choice(choices.next().unwrap());
        match choices.next().unwrap() {
            "X" => score += GameResult::Loss.to_points() + get_losing_choice(opponent).to_points(),
            "Y" => score += GameResult::Draw.to_points() + opponent.to_points(),
            "Z" => score += GameResult::Win.to_points() + get_winning_choice(opponent).to_points(),
            _ => exit(1),
        }
    }

    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(14));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(13));
    }
}
