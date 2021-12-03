use std::fs;

fn main() {
    println!("Hello World");

    let input = fs::read_to_string("input/d1.txt").unwrap();

    let mut prev_val = None;
    let mut increase_count = 0;
    for line in input.lines() {
        let val = line.parse::<i32>().unwrap();

        if let Some(prev_val) = prev_val {
            if val > prev_val {
                increase_count += 1;
            }
        }

        prev_val = Some(val);
    }

    println!("{}", increase_count);
}
