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
    let mut register: Vec<i32> = vec![];
    let mut screen: Vec<&str> = vec![];
    let mut value: i32 = 1;
    let mut cycle: i32 = 0;
    for line in lines {
        let instruction = build_command(line).unwrap();
        let (duration, added) = match instruction {
            Noop{duration , value} => (duration, value),
            Add{duration , value} => (duration, value)
        };
        for _ in 0..duration{
            register.push(value);
            let modulo = cycle % 40;
            let state: &str = if ((value-1)..(value+2)).contains(&modulo) {
                "#"
            } else {
                "."
            };
            screen.push(state);
            cycle += 1;
        }

        value += added;
    }
    println!("{}", cycle);
    for i in (0..240).step_by(40) {
        println!("{}", &screen[i..(i+40)].join(""));
    }
    
}
