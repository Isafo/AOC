use std::{fs, str::FromStr};

use glam::IVec2;

#[derive(Debug)]
enum Command {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl FromStr for Command {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line = s.split_whitespace().collect::<Vec<_>>();
        let val = line[1].parse().unwrap();
        Ok(match line[0] {
            "forward" => Command::Forward(val),
            "up" => Command::Up(val),
            "down" => Command::Down(val),
            _ => panic!("Invalid Command!"),
        })
    }
}

fn main() {
    let mut coordinates = IVec2::ZERO;
    let input = fs::read_to_string("input/d2.txt").unwrap();

    for command in input.lines().map(|line| line.parse::<Command>().unwrap()) {
        match command {
            Command::Forward(val) => coordinates.x += val,
            Command::Up(val) => coordinates.y -= val,
            Command::Down(val) => coordinates.y += val,
        }
    }

    let result = coordinates.x * coordinates.y;
    println!("{:?}", result);
}
