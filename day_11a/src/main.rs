use shunting::{ShuntingParser, MathContext};
use regex::Regex;
use lazy_static::lazy_static;
use std::string::String;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct Monkey {
    number: u64,
    items: VecDeque<u64>,
    operation: String,
    test: u64,
    pass: u64,
    fail: u64,
    inspections: u64
}

pub fn parse_monkey_data(raw: &str) -> Monkey{
    Monkey{
        number:  get_monkey_number(raw),
        items: get_starting_items(raw),
        operation: get_operation(raw),
        test: get_test_value(raw),
        pass: get_pass_monkey(raw),
        fail:  get_fail_monkey(raw),
        inspections: 0
    }

}

impl Monkey {
    pub fn calculate_value(&self, val: u64) -> u64 {
        let input = self.operation.clone().replace("old", &val.to_string());
        let expr = ShuntingParser::parse_str(&input).unwrap();
        MathContext::new().eval(&expr).unwrap() as u64
    }
}

pub fn get_starting_items(s: &str) -> VecDeque<u64> {
    lazy_static! {
        static ref DIRREG: Regex = Regex::new(r"Starting items: (.*)").unwrap();
    }
    let captures = DIRREG.captures(s).unwrap();
    let number_string: &str = captures.get(1).map_or("", |m| m.as_str());
    number_string.split(",").into_iter().map(|ex| ex.trim().parse().unwrap()).collect::<VecDeque<u64>>()
}

pub fn get_operation(s: &str) -> String {
    lazy_static! {
        static ref DIRREG: Regex = Regex::new(r"Operation: (.*)").unwrap();
    }
    let captures = DIRREG.captures(s).unwrap();
    String::from(captures.get(1).map_or("", |m| m.as_str().trim())).replace("new = ", "")
}

pub fn get_monkey_number(s: &str) -> u64 {
    lazy_static! {
        static ref MONKEE: Regex = Regex::new(r"Monkey (\d+):").unwrap();
    }
    let captures = MONKEE.captures(s).unwrap();
    captures.get(1).map_or("", |m| m.as_str()).parse().unwrap()
}

pub fn get_test_value(s: &str) -> u64{
    lazy_static! {
        static ref TEST: Regex = Regex::new(r"Test: divisible by (\d+)").unwrap();
    }
    let captures = TEST.captures(s).unwrap();
    captures.get(1).map_or(99, |c| c.as_str().trim().parse().unwrap())
}

pub fn get_pass_monkey(s: &str) -> u64 {
    lazy_static! {
        static ref TEST: Regex = Regex::new(r"If true: throw to monkey (\d+)").unwrap();
    }
    let captures = TEST.captures(s).unwrap();
    captures.get(1).map_or(99, |c| c.as_str().trim().parse().unwrap())
}

pub fn get_fail_monkey(s: &str) -> u64 {
    lazy_static! {
        static ref TEST: Regex = Regex::new(r"If false: throw to monkey (\d+)").unwrap();
    }
    let captures = TEST.captures(s).unwrap();
    captures.get(1).map_or(99, |c| c.as_str().trim().parse().unwrap())
}

fn main() {
    let lines = include_str!("../input.txt").split("\r\n\r\n");
    let mut monkey_stack: Vec<Rc<RefCell<Monkey>>>= vec![];
    for line in lines {
        let monkey = Rc::new(RefCell::new(parse_monkey_data(line)));
        monkey_stack.push(monkey.clone());
    }
    let worry_factor: u64 = monkey_stack.iter().map(|m| m.borrow().test).product();
    println!("Worry factor: {}", worry_factor);
    for _round in 0..10000 {
        // println!("Round: {}", round);
        for mk in monkey_stack.iter() {
            // println!("Monkey: {}", mk.borrow().number);
            loop {
                
                let current = mk.borrow_mut().items.pop_front();

                match current {
                    Some(it) => {
                        mk.borrow_mut().inspections += 1;
                        // println!("Item: {:?}", it);
                        //let value = ((mk.borrow().calculate_value(it) as f64) / 3.0).floor() as u64; part a
                        let value = mk.borrow().calculate_value(it) % worry_factor;
                        let worried = value % mk.borrow().test == 0;
                        // println!("New Level: {:?}", value);
                        let next_monkey: usize = if worried {
                            mk.borrow().pass as usize
                        } else {
                            mk.borrow().fail as usize
                        };
                        // println!("Sent to monkey: {:?}", next_monkey);
                        monkey_stack[next_monkey].borrow_mut().items.push_back(value);

                    },
                    None => break
                }
                
            }
        }
        
    }
    for mk in monkey_stack.iter() {
        println!("Monkey {}: {:?}. Insptections: {}",  mk.borrow().number, mk.borrow().items, mk.borrow().inspections);
    }
    let mut max_business: Vec<u64> = monkey_stack.iter().map(|mk| mk.borrow().inspections ).collect();
    max_business.sort();
    let mut monkey_business = 1;
    for _ in 0..2 {
        monkey_business *= max_business.pop().unwrap();
    }
    println!("Monkey Business: {:?}", monkey_business);

}
