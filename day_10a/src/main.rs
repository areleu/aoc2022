use self::Instruction::*;
use std::string::String;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Noop{ duration: i32, value: i32},
    Add{ duration: i32, value: i32}
}

pub fn build_command(s: &str) -> Result<Instruction, &'static str> {
    if String::from(s).contains("noop") {
        Ok(Noop{duration: 1, value: 0})
    } else if String::from(s).contains("addx") {
        let value: i32 = s.split(" ").collect::<Vec<&str>>()[1].parse().unwrap();
        Ok(Add{ duration: 2, value: value})
    } else {
        Err("Invalid option")
    }

}

fn main() {
    let lines = include_str!("../input.txt").lines();
    let interesting_signals = vec![20, 60, 100, 140, 180, 220];
    let mut register: Vec<i32> = vec![];
    let mut value: i32 = 1;
    for line in lines {
        let instruction = build_command(line).unwrap();
        let (duration, added) = match instruction {
            Noop{duration , value} => (duration, value),
            Add{duration , value} => (duration, value)
        };
        for _ in 0..duration{
            register.push(value)
        }

        value += added;
    }
    let mut output_value: i32 = 0;
    for is in interesting_signals.iter() {
        let signal_component: i32 = *is as i32;
        let signal_index: usize = *is -1  as usize;
        output_value += register[signal_index] * signal_component
    }
    println!("{:?}", output_value);
}
