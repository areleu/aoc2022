use std::str::FromStr;
use self::Direction::*;
use std::cell::RefCell;
use std::rc::Rc;
use itertools::Itertools;
use std::cmp; 

#[derive(Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down
}


#[derive(Debug, Clone)]
pub struct Knot {
    x: i32,
    y: i32,
    next: Option<Rc<RefCell<Knot>>>
}

impl Knot {
    pub fn change_position(&mut self, direction: &Direction) -> Vec<(i32, i32)> {
        let mut tail_positions: Vec<(i32, i32)> = vec![];
        match *direction {
            Left => {
                self.x -= 1
            },
            Right => {
                self.x += 1
            },
            Up => {
                self.y -= 1
            },
            Down => {
                self.y += 1
            }
        }
        let mut new_positions = match &self.next {
            Some(nxt) => {
                nxt.borrow_mut().follow(self.x, self.y)
            },
            None => vec![(0,0)]
        };
        tail_positions.append(&mut new_positions);
        tail_positions

    }

    pub fn follow(&mut self, parent_x: i32, parent_y: i32) -> Vec<(i32, i32)> {
        let distance = cmp::max((self.x - parent_x).abs(), (self.y - parent_y).abs());
        if distance > 1 {
           self.x += cmp::max(cmp::min(parent_x - self.x, 1), -1);
           self.y += cmp::max(cmp::min(parent_y - self.y, 1), -1);
        }
        
        let next_position: Vec<(i32, i32)>  = match &self.next {
            Some(nxt) => {
                nxt.borrow_mut().follow(self.x, self.y)
            },
            None => vec![(0,0)]
        };
        let positions: Vec<(i32, i32)> = match &self.next {
            Some(_) => {
                next_position
            },
            None => vec![(self.x, self.y)]
        };
        positions
    }
}

impl FromStr for Direction {

    type Err = ();

    fn from_str(input: &str) -> Result<Direction, Self::Err>  {
        match input {
            "L" => Ok(Left),
            "R" => Ok(Right),
            "U" => Ok(Up),
            "D" => Ok(Down),
             _  => Err(()),
        }
    }
}

fn main() {
    let string_lines = include_str!("../input.txt")
    .lines();
    let head = Rc::new(RefCell::new(Knot{x:0, y:0, next:None}));
    let mut knots: Vec<Rc<RefCell<Knot>>> = vec![head.clone()];
    let mut current_parent = head.clone();
    let mut position_vec: Vec<(i32, i32)> = vec![(head.borrow().x,head.borrow().y)];
    for _ in 0..9 {
        let new_knot = Rc::new(RefCell::new(Knot{x:0, y:0, next:None}));
        current_parent.borrow_mut().next = Some(new_knot.clone());
        current_parent = new_knot.clone();
        knots.push(new_knot);  
    }
    for line in string_lines {
        let line_vector = line.split(" ").collect::<Vec<&str>>();
        let (directionstr, number): (&str, u32) = (line_vector[0], line_vector[1].parse().unwrap());
        let direction = Direction::from_str(directionstr).unwrap();
        let head_ref = head.clone();
        for _ in 0..number {
            let mut new_pos = head_ref.borrow_mut().change_position(&direction);
            position_vec.append(&mut new_pos);
        }
    }
    println!("{}", position_vec.iter().unique().collect::<Vec<&(i32, i32)>>().len());
}
