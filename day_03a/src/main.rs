pub struct Rucksack<'a> {
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
    let mut ch = Err('0');
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
impl<'a> From<&'a str> for Rucksack<'a> {
    fn from(s: &'a str) -> Self {
        let left: Compartment = Compartment{items: &s[0..s.len() /2]};
        let right: Compartment = Compartment{items: &s[s.len() /2..s.len()]};        
        Rucksack {
            left: left,
            right: right,
        } 
    }
}
fn main() {
    let string_lines = include_str!("../input.txt")
                                   .lines();
    let mut total: i32 = 0;
    for line in string_lines {
        let rucksack = Rucksack::from(line);
        let common: char = rucksack.common();
        let value: i32 = assign_value(common);

        total = total + value;
    }
    println!("{}",total)
}
