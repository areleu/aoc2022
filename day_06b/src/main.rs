fn main() {
    let datastream = include_str!("../input.txt").chars().collect::<Vec<char>>();
    let mut counts: Vec<i32> = Vec::new();
    'outer: for s in 0..(datastream.len()-3){
        let window = &datastream[s..(s+14)];
        for c in window.into_iter() {
            let count = window.iter().filter(|&n| *n == *c).count() as i32;
            println!("{:?}", count);
            counts.push(count);
            if counts.len() == 14 {
                let total: i32 = counts.iter().sum();
                if total == 14 {
                    println!("{:?}", s + 14);
                    break 'outer;
                }
                counts.drain(..);
            }
            
        }
        
    }
}
