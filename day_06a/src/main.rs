fn main() {
    let datastream = include_str!("../input.txt").chars().collect::<Vec<char>>();
    let mut counts: Vec<i32> = Vec::new();
    'outer: for s in 0..(datastream.len()-3){
        let window = &datastream[s..(s+4)];
        for c in window.into_iter() {
            let count = window.iter().filter(|&n| *n == *c).count() as i32;
            counts.push(count);
            if counts.len() == 4 {
                let total: i32 = counts.iter().sum();
                if total == 4 {
                    println!("{:?}", s + 4);
                    break 'outer;
                    
                }
                counts.drain(..);
            }
            
        }
        
    }
}
