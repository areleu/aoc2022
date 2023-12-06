pub fn check_containment(first: &mut Vec<i32>, second: &mut Vec<i32>) -> bool {
    let (shortest, longest) = if first.len() > second.len() {
        let longest = first.to_vec();
        let shortest = second.to_vec();
        (shortest, longest) 
    } else if second.len() > first.len() {
        let longest = second.to_vec();
        let shortest = first.to_vec();
        (shortest, longest) 
    } else {
        let longest = first.to_vec();
        let shortest = second.to_vec();  
        (shortest, longest)    
    };
    let overlap: Vec<i32> = longest.into_iter().filter(|n| shortest.contains(n)).collect();
    if overlap.len() != 0 {
        true
    } else {
        false
    }
}

pub fn construct_assignments(pair: &str) -> (Vec<i32>, Vec<i32>){
    let mut assignments = pair.split(",");
    let mut first = assignments.next().unwrap().split("-");
    let mut second = assignments.next().unwrap().split("-");

    let first_assignment: Vec<i32> = (first
                                      .next()
                                      .unwrap()
                                      .parse::<i32>().unwrap()..
                                      first.next()
                                      .unwrap()
                                      .parse::<i32>().unwrap() +1)
                                      .collect();

    let second_assignment: Vec<i32> = (second
                                        .next()
                                        .unwrap()
                                        .parse::<i32>().unwrap()..
                                        second.next()
                                        .unwrap()
                                        .parse::<i32>().unwrap()  +1 )
                                        .collect();
    (first_assignment, second_assignment)
}

fn main() {
    let string_lines = include_str!("../input.txt")
                                   .lines();
    let mut total: i32 = 0;
    for line in string_lines {
        let (mut first, mut second) = construct_assignments(line);
        let containment = check_containment(&mut first, &mut second) as i32; 
        total = total + containment
        
    }
    println!("{:?}", total);
}
