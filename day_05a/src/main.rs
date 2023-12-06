use lazy_static::lazy_static;
use regex::Regex;

#[derive(Clone, Debug)]
pub struct Platform{
    containers: Vec<Container>,
}

#[derive(Clone, Debug)]
pub struct Container {
    items: Vec<char>
}

#[derive(Clone, Debug)]
pub struct Movement{
    amount: i32,
    from: i32,
    to: i32
}

pub trait Execute {
    fn execute(&mut self, movement: &Movement);
}

pub fn str_strip_numbers(s: &str) -> Vec<char> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\d+").unwrap();
    }
    RE.find_iter(s)
        .filter_map(|digits| Some(digits.as_str())).map(| c | c.chars().nth(0).unwrap()).collect()
}

pub fn find_movement_numbers(s: &str) -> (i32, i32, i32) {
    lazy_static! {
        static ref REMOV: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    }
    let captures = REMOV.captures(s).unwrap();
    (captures.get(1).map_or(0, |m| m.as_str().parse().unwrap()),
    captures.get(2).map_or(0, |m| m.as_str().parse().unwrap()),
    captures.get(3).map_or(0, |m| m.as_str().parse().unwrap()))
}

impl From<&str> for Platform {
    fn from(string: &str) -> Self {
        let mut state_descriptor: Vec<&str> = string.lines().collect();
        let bases  = state_descriptor.pop().unwrap();
        state_descriptor = state_descriptor.into_iter().rev().collect();
        let base_amounts = str_strip_numbers(bases);
        let mut containers = Vec::<Container>::new();
        for _base in &base_amounts {
            let container_base = Container{
                items: Vec::<char>::new()
            };
            containers.push(container_base);
        }
        for level in &state_descriptor {
            let bytes: &[u8] = level.as_bytes();
            for (i, base) in base_amounts.iter().enumerate()  {
                let index = bases.find(*base).unwrap() as usize;
                let current_byte = bytes[index] as char;
                if current_byte != ' ' {
                    containers[i].items.push(current_byte)
                }
            }
        }
        Platform {
            containers: containers
        }
        
    }

}

impl Execute for Platform {
    fn execute(&mut self, movement: &Movement) {
        let from = movement.from as usize -1;
        let to = movement.to as usize -1;
        let final_length = self.containers[from].items.len() - (movement.amount as usize);
        let mut buffer = self.containers[from].items.split_off(final_length).into_iter().rev().collect();
        println!("{:?}", buffer);
        self.containers[to].items.append(&mut buffer);
        println!("{:?}", self.containers);
    }

}

impl From<&str> for Movement {
    fn from(string: &str) -> Self {
        let (amount, from, to) = find_movement_numbers(string);
        Movement {
            amount: amount,
            from: from,
            to: to
        }
    }

}

fn main() {
    let mut string_lines = include_str!("../input.txt").split("\r\n\r\n");
    let state = string_lines.next().unwrap();
    let movements = string_lines.next().unwrap().lines();
    let mut platform = Platform::from(state);
    println!("{:?}", platform.containers);
    for mov in movements {
        let current_mov = Movement::from(mov);
        println!("{:?}", current_mov);
        platform.execute(&current_mov);
    }
    let mut message = String::new();
    for stack in &platform.containers {
        message.push(stack.items[stack.items.len() - 1])
    }
    println!("{:?}", platform.containers);
    println!("{:?}", message);
}
