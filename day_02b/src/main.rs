

use self::Hand::*;
use self::Outcome::*;

#[derive(Debug, Eq, PartialEq)]
pub enum Outcome {
    Win,
    Lose,
    Draw
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Hand {
    Rock,
    Paper,
    Scissors
}

pub trait Beats {
    fn beats(&self) -> Self;
    fn beaten_by(&self) -> Self;
}

impl Beats for Hand {
    fn beats(&self) -> Self {
        match *self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper

        }
    }

    fn beaten_by(&self) -> Self {
        match *self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock
        }       
    }
}

impl Outcome {
    fn value(&self) -> i32 {
        match *self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0
        }
    }
}

impl Hand {
    fn value(&self) -> i32 {
        match *self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3
        }
    }
}

pub fn play(own: Hand, other: Hand) -> Outcome {
    let own_beats = own.beats();
    let other_beats = other.beats();

    match (own_beats, other_beats) {
        _ if own_beats == other => Win,
        _ if other_beats == own => Lose,
        _                       => Draw
    }
}

pub fn decode_hand(code: &str) -> Hand {
    match code {
        _ if code == "A" => Rock,
        _ if code == "B" => Paper,
        _ if code == "C" => Scissors,
        &_ => todo!()
    }
}

pub fn decode_outcome(code: &str) -> Outcome {
    match code {
        _ if code == "X" => Lose,
        _ if code == "Y" => Draw,
        _ if code == "Z" => Win,
        &_ => todo!()
    }
}

pub fn cheat(outcome: &Outcome, other: Hand) -> Hand {
    match outcome {
        Win => other.beaten_by(),
        Lose => other.beats(),
        Draw => other
    }
}
fn main() {
    let string_lines = include_str!("../input.txt")
                            .lines();
    let mut score: i32 = 0;
    for line in string_lines {
        let hands: Vec<&str> = line.split_whitespace().collect();
        let desired_outcome: Outcome = decode_outcome(hands[1]);
        let other_hand: Hand = decode_hand(hands[0]);

        let own_hand: Hand = cheat(&desired_outcome, other_hand);

        score = score + desired_outcome.value() + own_hand.value()
        }
        println!("{}",score)
}
