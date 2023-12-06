use ndarray::{Array2, ArrayView, s};
use ndarray_stats::QuantileExt;
fn main() {

    let string_lines = include_str!("../input.txt")
    .lines();
    const RADIX: u32 = 10;
    let mut array_vector: Vec<Vec<u32>> = vec![];
    let mut nrows = 0;
    let mut data = Vec::new();
    let mut zeroes = Vec::new();
    for line in string_lines {
        let current_vector: Vec<u32> = line.chars().map(|c| c.to_digit(RADIX).unwrap()).collect();
        data.extend_from_slice(&current_vector);
        zeroes.extend_from_slice(&vec![0; current_vector.len()]);
        array_vector.push(current_vector);
        nrows += 1;
    }
    let ncols = array_vector.first().map_or(0, |row| row.len());
    let arr = Array2::from_shape_vec((nrows, ncols), data).unwrap();
    let mut bin = Array2::from_shape_vec((nrows, ncols), zeroes).unwrap();
    println!("{:?}", arr);
    let slice = arr.slice(s![0, ..]);
    
    for ((x, y), value) in arr.indexed_iter() {
        if !((x == 0) | (y == 0) | (x == ncols - 1) | (y == nrows - 1)){
            let mut corner_check: Vec<bool> = vec![];
            let mut viewing_account: Vec<u32> = vec![];
            let left_slice = arr.slice(s![x,0..y]).to_vec();
            let viewing_boundary = left_slice.iter().rposition(|a| *a >= *value);
            let viewing_score: u32 = match viewing_boundary{
                Some(n) => (left_slice.len() - n) as u32,
                None => left_slice.len() as u32
            };
            viewing_account.push(viewing_score);
            let right_slice = arr.slice(s![x,y+1..]);
            let viewing_boundary = right_slice.iter().position(|a| *a >= *value);
            let viewing_score: u32 = match viewing_boundary{
                Some(n) => (n+1) as u32,
                None => right_slice.len() as u32
            };
            viewing_account.push(viewing_score);
            let top_slice = arr.slice(s![0..x,y]);
            let viewing_boundary = top_slice.iter().rposition(|a| *a >= *value);
            let viewing_score: u32 = match viewing_boundary{
                Some(n) => (top_slice.len() - n) as u32,
                None => top_slice.len() as u32
            };
            viewing_account.push(viewing_score);
            let bot_slice = arr.slice(s![x+1..,y]);
            let viewing_boundary = bot_slice.iter().position(|a| *a >= *value);
            let viewing_score: u32 = match viewing_boundary{
                Some(n) => (n+1) as u32,
                None => bot_slice.len() as u32
            };
            viewing_account.push(viewing_score);
            let score: u32 = viewing_account.iter().product();
            println!("{:?}", score);
            if corner_check.iter().all(|a| *a == true) {
                bin[(x,y)] = score;

            }
        } 
    }
    println!("{:?}", bin);
    println!("{:?}", bin[bin.argmax().unwrap()]);
    
}
