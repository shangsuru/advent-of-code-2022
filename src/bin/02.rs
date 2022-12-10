use std::str::FromStr;

enum GameResult {
    Loss,
    Draw,
    Win,
}

impl GameResult {
    fn to_points(&self) -> u32 {
        match self {
            GameResult::Win => 6,
            GameResult::Draw => 3,
            GameResult::Loss => 0,
        }
    }

    fn matching_choice(self, opponent: Choice) -> Choice {
        match self {
            GameResult::Loss => opponent.losing_choice(),
            GameResult::Draw => opponent.drawing_choice(),
            GameResult::Win => opponent.winning_choice(),
        }
    }
}

impl TryFrom<char> for GameResult {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'X' => Ok(GameResult::Loss),
            'Y' => Ok(GameResult::Draw),
            'Z' => Ok(GameResult::Win),
            _ => Err("Expected X, Y or Z to encode Loss, Draw or Win respectively".to_string()),
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn to_points(self) -> u32 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }

    fn against(self, opponent: Choice) -> GameResult {
        if self == opponent.losing_choice() {
            return GameResult::Loss;
        }
        if self == opponent.winning_choice() {
            return GameResult::Win;
        }

        GameResult::Draw
    }

    fn winning_choice(self) -> Self {
        match self {
            Choice::Rock => Choice::Paper,
            Choice::Paper => Choice::Scissors,
            Choice::Scissors => Choice::Rock,
        }
    }

    fn losing_choice(self) -> Self {
        match self {
            Choice::Rock => Choice::Scissors,
            Choice::Paper => Choice::Rock,
            Choice::Scissors => Choice::Paper,
        }
    }

    fn drawing_choice(self) -> Self {
        self
    }
}

impl TryFrom<char> for Choice {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' | 'X' => Ok(Choice::Rock),
            'B' | 'Y' => Ok(Choice::Paper),
            'C' | 'Z' => Ok(Choice::Scissors),
            _ => Err(
                "Expected A,B,C or X,Y,Z respectively to encode Rock, Paper, Scissors".to_string(),
            ),
        }
    }
}

struct Round {
    player: Choice,
    opponent: Choice,
}

impl Round {
    fn outcome(self) -> GameResult {
        self.player.against(self.opponent)
    }

    fn score(self) -> u32 {
        self.player.to_points() + self.outcome().to_points()
    }
}

impl FromStr for Round {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let (Some(opponent), Some(' '), Some(player), None) = (chars.next(), chars.next(), chars.next(), chars.next()) else {
            return Err("Expected <opponent> <player>, got {s:?}".to_string());
        };

        Ok(Self {
            player: player.try_into()?,
            opponent: opponent.try_into()?,
        })
    }
}

struct Round2(Round);

impl FromStr for Round2 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let (Some(opponent), Some(' '), Some(result), None) = (chars.next(), chars.next(), chars.next(), chars.next()) else {
            return Err("Expected <opponent> <player>, got {s:?}".to_string());
        };

        let opponent = Choice::try_from(opponent)?;
        let result = GameResult::try_from(result)?;
        let player = result.matching_choice(opponent);
        Ok(Self(Round { player, opponent }))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut score = 0;
    for round in input.lines().map(Round::from_str) {
        score += round.unwrap().score();
    }

    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut score: u32 = 0;

    for round in input.lines().map(Round2::from_str) {
        score += round.unwrap().0.score();
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
