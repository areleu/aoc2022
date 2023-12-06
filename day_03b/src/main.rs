pub struct Rucksack<'a> {
    all: &'a str,
    left: Compartment<'a>,
    right: Compartment<'a>
}

pub struct Compartment<'a> {
    items: &'a str,
}

pub trait Contents {
    fn common(&self) -> char;
}

impl Contents for Rucksack<'_> {
    fn common(&self) -> char{
        let result: Result<char, char> = find_common_character(&self.left, &self.right);
        match result {
            Ok(ch) => ch,
            Err(_) => panic!("No common character found")
        }

    }
}

pub fn find_common_character(left: &Compartment, right: &Compartment) -> Result<char, char> {
    let left_string: &str = left.items;
    let right_string: String = String::from(right.items);
    let mut ch = Err('1');
    for c in left_string.chars() {
        if right_string.contains(c) {
            ch = Ok(c);
            break;
        } 
    }
    ch
}

pub fn assign_value(c: char) -> i32{
    let mut position: i32 = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
                   .chars()
                   .position(|ch| ch == c)
                   .unwrap() as i32;
    position = position + 1;
    position
}

fn find_common_item(group: &mut Vec<Rucksack>) -> Result<char, char> {
    let first_rucksack: Rucksack = group.pop().unwrap();
    let mut existing_items: Vec<char> = first_rucksack.all.chars().collect();
    for rs in group {
        let rs_string: String = String::from(rs.all);
        existing_items = existing_items.into_iter().filter(|c| rs_string.contains(*c)).collect::<Vec<char>>();
    }
    existing_items.dedup();
    if existing_items.len() != 1 {
        Err('1')
    } else {
        Ok(existing_items.pop().unwrap())
    }
}

impl<'a> From<&'a str> for Rucksack<'a> {
    fn from(s: &'a str) -> Self {
        let left: Compartment = Compartment{items: &s[0..s.len() /2]};
        let right: Compartment = Compartment{items: &s[s.len() /2..s.len()]};        
        Rucksack {
            all: s,
            left: left,
            right: right,
        } 
    }
}

fn main() {
    let string_lines = include_str!("../input.txt")
                                   .lines();
    let mut total: i32 = 0;
    let mut all_rucksacks: Vec<Rucksack> = Vec::new();
    for line in string_lines {
        all_rucksacks.push(Rucksack::from(line));
        if all_rucksacks.len() == 3 {
            let common = find_common_item(&mut all_rucksacks).unwrap();
            let value: i32 = assign_value(common);
            total = total + value;
            all_rucksacks.drain(..);
        }

    }
    println!("{}",total)
}
