use std::fs;

fn get_input_file(path: &str) -> Vec<String> {
	let contents = fs::read_to_string(path).expect("Unable to read input file");
	return contents.lines().map(|s: &str| s.to_string()).collect();
}

fn part_1(input: Vec<String>) -> i32 {
	let input_length = input.len();

	let mut total_pos_bit = vec![0; input[0].len()];

	for val in input {
		for (i, bit) in val.chars().enumerate() {
			if bit == '1' {
				total_pos_bit[i] = total_pos_bit[i] + 1;
			}
		}
	}

	let gamma_rate_binary = total_pos_bit
		.iter()
		.map(|&x| if x > input_length / 2 { '1' } else { '0' })
		.collect::<String>();

	let epsilon_rate_binary = gamma_rate_binary
		.chars()
		.map(|c| if c == '1' { '0' } else { '1' })
		.collect::<String>();

	let gamma_rate_integer = i32::from_str_radix(&gamma_rate_binary, 2).unwrap();
	let epsilon_rate_integer = i32::from_str_radix(&epsilon_rate_binary, 2).unwrap();

	return gamma_rate_integer * epsilon_rate_integer;
}

fn scrubber(input: Vec<String>, use_abundant: bool) -> String {
	let bit_string_length = input[0].len();

	let mut lines_remaining = input.to_vec();

	for i in 0..bit_string_length {
		let mut ones = Vec::new();
		let mut zeros = Vec::new();

		for line in lines_remaining {
			let bit = line.chars().nth(i).unwrap();

			if bit == '1' {
				ones.push(line);
			} else {
				zeros.push(line);
			}
		}

		let mut discard = &zeros;

		if ones.len() >= zeros.len() {
			lines_remaining = ones;
		} else {
			lines_remaining = zeros;
			discard = &ones;
		}

		if !use_abundant {
			lines_remaining = discard.to_vec();
		}

		if lines_remaining.len() == 1 {
			return lines_remaining[0].to_string();
		}
	}

	return String::new();
}

fn part_2(input: Vec<String>) -> i32 {
	let o2_rating = scrubber(input.to_vec(), true);
	let co2_rating = scrubber(input.to_vec(), false);
	let o2_rating_integer = i32::from_str_radix(&o2_rating, 2).unwrap();
	let co2_rating_integer = i32::from_str_radix(&co2_rating, 2).unwrap();

	return o2_rating_integer * co2_rating_integer;
}

pub fn main() {
	println!(
		"Day 3.1: {}",
		part_1(get_input_file("inputs/day_03/input.txt"))
	);

	println!(
		"Day 3.2: {}",
		part_2(get_input_file("inputs/day_03/input.txt"))
	);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part_1() {
		assert_eq!(part_1(get_input_file("inputs/day_03/input.txt")), 3148794);
	}

	#[test]
	fn test_part_2() {
		assert_eq!(part_2(get_input_file("inputs/day_03/input.txt")), 2795310);
	}
}
