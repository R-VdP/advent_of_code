use std::fs;
use std::str::FromStr;

#[repr(u64)]
#[derive(Debug, PartialEq, Eq)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Move {
    fn parse(s: &str) -> Move {
        FromStr::from_str(s).unwrap_or_else(|msg| panic!("Error parsing: {}", msg))
    }

    fn to_score(&self) -> u64 {
        *self as u64
    }
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            otherwise => Err(format!("Wrong input! ({})", otherwise)),
        }
    }
}

#[derive(Debug)]
struct Round {
    my_move: Move,
    their_move: Move,
}

enum Outcome {
    Win,
    Tie,
    Lose,
}

impl Outcome {
    fn invert(&self) -> Self {
        match self {
            Outcome::Win => Outcome::Lose,
            Outcome::Tie => Outcome::Tie,
            Outcome::Lose => Outcome::Win,
        }
    }
}

impl Round {
    fn to_score(&self) -> u64 {
        (match self.play() {
            Outcome::Lose => 0,
            Outcome::Tie => 3,
            Outcome::Win => 6,
        }) + self.my_move.to_score()
    }

    fn play(&self) -> Outcome {
        fn against(m1: &Move, m2: &Move) -> Outcome {
            match (m1, m2) {
                (m1, m2) if m1 == m2 => Outcome::Tie,
                (Move::Rock, Move::Paper) => Outcome::Lose,
                (Move::Rock, Move::Scissors) => Outcome::Win,
                (Move::Paper, Move::Scissors) => Outcome::Lose,
                _ => against(m2, m1).invert(),
            }
        }
        against(&self.my_move, &self.their_move)
    }
}

fn parse_moves(input: &str) -> Vec<Round> {
    input
        .lines()
        .collect::<Vec<&str>>()
        .into_iter()
        .flat_map(|round| {
            let mut moves = round.split(" ");
            moves
                .next()
                .into_iter()
                .zip(moves.next().into_iter())
                .map(|(m1, m2)| Round {
                    their_move: Move::parse(m1),
                    my_move: Move::parse(m2),
                })
                .take(1)
                .next()
        })
        .collect()
}

fn get_score(input: &str) -> u64 {
    parse_moves(input).iter().map(Round::to_score).sum()
}

fn get_input() -> String {
    fs::read_to_string("./input.txt").unwrap_or_default()
}

fn main() {
    println!(
        "{:?}",
        parse_moves(&get_input())
            .iter()
            .map(Round::to_score)
            .collect::<Vec<_>>()
    );
    println!("{:?}", get_score(&get_input()));
}

#[allow(dead_code)]
const EXAMPLE_INPUT: &'static str = "
A Y
B X
C Z
";

#[allow(dead_code)]
const FISRT_TEN_INPUT: &'static str = "
C X 1
C X 1 2
A Y 8 10
C X 1 11
B Y 5 16
A X 4 20
A Z 9 29
B Y 5 34
C Z 6 40
C Z 6 46
";

#[test]
fn test_example() {
    assert_eq!(get_score(EXAMPLE_INPUT), 15)
}

#[test]
fn test_first_ten() {
    assert_eq!(get_score(FISRT_TEN_INPUT), 58)
}

#[test]
fn test_input_length() {
    assert_eq!(parse_moves(&get_input()).len(), 2500);
    assert_eq!(
        parse_moves(&get_input())
            .iter()
            .map(Round::to_score)
            .count(),
        2500
    );
}

#[test]
fn test_perms() {
    assert_eq!(get_score(&"A X"), 4);
    assert_eq!(get_score(&"A Y"), 8);
    assert_eq!(get_score(&"A Z"), 3);
    assert_eq!(get_score(&"B X"), 1);
    assert_eq!(get_score(&"B Y"), 5);
    assert_eq!(get_score(&"B Z"), 9);
    assert_eq!(get_score(&"C X"), 7);
    assert_eq!(get_score(&"C Y"), 2);
    assert_eq!(get_score(&"C Z"), 6);
}
