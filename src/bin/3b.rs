use std::{fs, ops::RangeBounds, thread::current};

use bit_field::BitField;

const BIT_COUNT: usize = 12;

struct Diagnostics {
    data: [i32; BIT_COUNT],
}

fn get_numbers(raw_data: &mut Vec<u32>, s: &str) {
    let val = u32::from_str_radix(s, 2).unwrap();
    raw_data.push(val);
}

fn count_bits(result: &mut Diagnostics, raw_data: &[u32]) {
    for val in raw_data {
        for (index, data_it) in result.data.iter_mut().enumerate() {
            let bit = val.get_bit((BIT_COUNT - 1) - index);
            *data_it += if bit { 1 } else { -1 };
        }
    }
}

fn evaluate_oxygen(raw_data: Vec<u32>) -> u32 {
    let mut oxygen = raw_data;

    for bit_index in 0..BIT_COUNT {
        let mut result = Diagnostics {
            data: [0; BIT_COUNT],
        };

        count_bits(&mut result, &oxygen);

        let bitValue = if result.data[bit_index] >= 0 {
            true
        } else {
            false
        };

        oxygen = oxygen
            .iter()
            .copied()
            .filter(|val| val.get_bit((BIT_COUNT - 1) - bit_index) == bitValue)
            .collect::<Vec<_>>();

        if oxygen.len() == 1 {
            break;
        }
    }

    oxygen[0]
}

fn evaluate_carbon(raw_data: Vec<u32>) -> u32 {
    let mut carbon = raw_data;

    for bit_index in 0..BIT_COUNT {
        let mut result = Diagnostics {
            data: [0; BIT_COUNT],
        };

        count_bits(&mut result, &carbon);

        let bitValue = if result.data[bit_index] < 0 {
            true
        } else {
            false
        };

        carbon = carbon
            .iter()
            .copied()
            .filter(|val| val.get_bit((BIT_COUNT - 1) - bit_index) == bitValue)
            .collect::<Vec<_>>();

        if carbon.len() == 1 {
            break;
        }
    }

    carbon[0]
}

fn main() {
    let input = fs::read_to_string("input/d3.txt").unwrap();

    let mut raw_data: Vec<u32> = Vec::new();
    for line in input.lines() {
        get_numbers(&mut raw_data, line);
    }

    let oxygen = evaluate_oxygen(raw_data.clone());
    let carbon = evaluate_carbon(raw_data.clone());
    let life_support = oxygen * carbon;

    println!("{}", life_support);
}
