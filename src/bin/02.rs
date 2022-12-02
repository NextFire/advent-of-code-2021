use std::{cmp::Ordering, fs};

#[derive(Debug, Eq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl From<char> for Choice {
    fn from(val: char) -> Self {
        match val {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => panic!(),
        }
    }
}

impl Ord for Choice {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Choice::Rock, Choice::Rock) => Ordering::Equal,
            (Choice::Rock, Choice::Paper) => Ordering::Less,
            (Choice::Rock, Choice::Scissors) => Ordering::Greater,
            (Choice::Paper, Choice::Rock) => Ordering::Greater,
            (Choice::Paper, Choice::Paper) => Ordering::Equal,
            (Choice::Paper, Choice::Scissors) => Ordering::Less,
            (Choice::Scissors, Choice::Rock) => Ordering::Less,
            (Choice::Scissors, Choice::Paper) => Ordering::Greater,
            (Choice::Scissors, Choice::Scissors) => Ordering::Equal,
        }
    }
}

impl PartialOrd for Choice {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Choice {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Choice {
    fn points(&self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

#[derive(Debug)]
struct Round {
    opponent: Choice,
    player: Choice,
}

impl Round {
    fn score(&self) -> i32 {
        let mut score = 0;
        if self.player > self.opponent {
            score += 6;
        } else if self.player == self.opponent {
            score += 3;
        } else {
            score += 0;
        }
        score += self.player.points();
        score
    }
}

#[derive(Debug)]
enum Result {
    Lose,
    Draw,
    Win,
}

impl From<char> for Result {
    fn from(val: char) -> Self {
        match val {
            'X' => Result::Lose,
            'Y' => Result::Draw,
            'Z' => Result::Win,
            _ => panic!(),
        }
    }
}

impl Result {
    fn computed(&self, opponent: &Choice) -> Choice {
        match (self, opponent) {
            (Result::Lose, Choice::Rock) => Choice::Scissors,
            (Result::Lose, Choice::Paper) => Choice::Rock,
            (Result::Lose, Choice::Scissors) => Choice::Paper,
            (Result::Draw, Choice::Rock) => Choice::Rock,
            (Result::Draw, Choice::Paper) => Choice::Paper,
            (Result::Draw, Choice::Scissors) => Choice::Scissors,
            (Result::Win, Choice::Rock) => Choice::Paper,
            (Result::Win, Choice::Paper) => Choice::Scissors,
            (Result::Win, Choice::Scissors) => Choice::Rock,
        }
    }
}

fn main() {
    let input = fs::read_to_string("inputs/02.txt").unwrap();

    let rounds = input.lines().map(|l| {
        let chars: Vec<_> = l.chars().collect();
        let opponent = Choice::from(chars[0]);
        let player = Choice::from(chars[2]);
        let round = Round { opponent, player };
        round
    });
    let total: i32 = rounds.map(|r| r.score()).sum();
    println!("{:?}", total);

    let rounds = input.lines().map(|l| {
        let chars: Vec<_> = l.chars().collect();
        let opponent = Choice::from(chars[0]);
        let result = Result::from(chars[2]);
        let player = result.computed(&opponent);
        let round = Round { opponent, player };
        round
    });
    let total: i32 = rounds.map(|r| r.score()).sum();
    println!("{:?}", total);
}
