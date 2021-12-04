use std::fs;

use bit_field::BitField;

const BIT_COUNT: usize = 12;

struct Diagnostics {
    data: [i32; BIT_COUNT],
}

fn count_bits(result: &mut Diagnostics, s: &str) {
    let val = u32::from_str_radix(s, 2).unwrap();
    for (index, data_it) in result.data.iter_mut().enumerate() {
        let bit = val.get_bit(index);
        *data_it += if bit { 1 } else { -1 };
    }
}

fn main() {
    let input = fs::read_to_string("input/d3.txt").unwrap();

    let mut diagnostics = Diagnostics {
        data: [0; BIT_COUNT],
    };

    for line in input.lines() {
        count_bits(&mut diagnostics, line);
    }

    let mut result_bit: u32 = 0;
    let mut index: u32 = 0;
    for data_it in diagnostics.data.iter() {
        if *data_it > 0 {
            result_bit += 2u32.pow(index);
        }

        index += 1;
    }
    let gamma_rate = result_bit.get_bits(0..BIT_COUNT);
    let epsilon_rate = (!gamma_rate).get_bits(0..BIT_COUNT);
    let power_usage = gamma_rate * epsilon_rate;

    println!("{}", power_usage);
}
