use ndarray::{Array2, ArrayView, s};
fn main() {

    let string_lines = include_str!("../input.txt")
    .lines();
    const RADIX: u32 = 10;
    let mut array_vector: Vec<Vec<u32>> = vec![];
    let mut nrows = 0;
    let mut data = Vec::new();
    let mut ones = Vec::new();
    for line in string_lines {
        let current_vector: Vec<u32> = line.chars().map(|c| c.to_digit(RADIX).unwrap()).collect();
        data.extend_from_slice(&current_vector);
        ones.extend_from_slice(&vec![1; current_vector.len()]);
        array_vector.push(current_vector);
        nrows += 1;
    }
    let ncols = array_vector.first().map_or(0, |row| row.len());
    let arr = Array2::from_shape_vec((nrows, ncols), data).unwrap();
    let mut bin = Array2::from_shape_vec((nrows, ncols), ones).unwrap();
    println!("{:?}", arr);
    let slice = arr.slice(s![0, ..]);
    
    for ((x, y), value) in arr.indexed_iter() {
        if !((x == 0) | (y == 0) | (x == ncols - 1) | (y == nrows - 1)){
            let mut corner_check: Vec<bool> = vec![];
            let left_slice = arr.slice(s![x,0..y]);
            corner_check.push(left_slice.to_vec().iter().any(|a| a >= value));
            let right_slice = arr.slice(s![x,y+1..]);
            corner_check.push(right_slice.to_vec().iter().any(|a| a >= value));
            let top_slice = arr.slice(s![0..x,y]);
            corner_check.push(top_slice.to_vec().iter().any(|a| a >= value));
            let bot_slice = arr.slice(s![x+1..,y]);
            corner_check.push(bot_slice.to_vec().iter().any(|a| a >= value));
            if corner_check.iter().all(|a| *a == true) {
                bin[(x,y)] = 0;

            }
        } 
    }
    println!("{:?}", bin.sum());
    
}
