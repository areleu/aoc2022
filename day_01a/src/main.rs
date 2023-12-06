fn main() {
    let string_lines = include_str!("../input.txt")
                                    .lines() ;
    let mut total_calories: Vec<i32> = Vec::new();
    let mut calories: i32 = 0 ;
    for line in string_lines {
        let current_value: Result<i32, _>= line.parse();
        let not_new_line = match current_value {
            Ok(_) => true,
            Err(_) => false
        } ;
        if not_new_line {
            calories = calories + current_value.unwrap();
        } else  {
            total_calories.push(calories);
            calories = 0;
        }
    }
    total_calories.push(calories);
    let max_value = total_calories.iter().max_by(|a, b| a.partial_cmp(b).unwrap());
    println!("{}", max_value.unwrap())
}
