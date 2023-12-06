

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
}

impl Beats for Hand {
    fn beats(&self) -> Self {
        match *self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper

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
        _ if code == "X" => Rock,
        _ if code == "Y" => Paper,
        _ if code == "Z" => Scissors,
        _ if code == "A" => Rock,
        _ if code == "B" => Paper,
        _ if code == "C" => Scissors,
        &_ => todo!()
    }
}

fn main() {
    let string_lines = include_str!("../input.txt")
                            .lines();
    let mut score: i32 = 0;
    for line in string_lines {
        let hands: Vec<&str> = line.split_whitespace().collect();
        let own_hand: Hand = decode_hand(hands[1]);
        let other_hand: Hand = decode_hand(hands[0]);
        let outcome: Outcome = play(own_hand, other_hand);

        score = score + outcome.value() + own_hand.value()
        }
        println!("{}",score)

}
