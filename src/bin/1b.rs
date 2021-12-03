use std::fs;

fn main() {
    println!("Hello World");

    let input = fs::read_to_string("input/d1.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut prev_val = None;
    let mut increase_count = 0;
    for it in 2..input.len() {
        let mut sum = input[it - 2];
        sum += input[it - 1];
        sum += input[it];

        if let Some(prev_val) = prev_val {
            if sum > prev_val {
                increase_count += 1;
            }
        }

        prev_val = Some(sum);
    }

    println!("{}", increase_count);
}
