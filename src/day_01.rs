use std::fs;

fn get_input_file(path: &str) -> Vec<u32> {
	let contents = fs::read_to_string(path).expect("Unable to read input file");

	let input: Vec<u32> = contents
		.lines()
		.map(|s: &str| s.parse::<u32>().unwrap_or(0))
		.collect();

	input
}

fn part_1(input: Vec<u32>) -> u32 {
	let mut increases = 0;

	for n in 1..input.len() {
		if input[n] > input[n - 1] {
			increases = increases + 1;
		}
	}

	return increases;
}

fn part_2(input: Vec<u32>) -> u32 {
	let mut increases = 0;
	let mut previous_window: u32 = input.windows(3).next().unwrap().iter().sum();

	for window in input.windows(3) {
		let current_window = window.iter().sum();

		if current_window > previous_window {
			increases = increases + 1;
		}

		previous_window = current_window;
	}

	return increases;
}

pub fn main() {
	println!(
		"Day 1.1: {}",
		part_1(get_input_file("inputs/day_01/input.txt"))
	);

	println!(
		"Day 1.2: {}",
		part_2(get_input_file("inputs/day_01/input.txt"))
	);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part_1() {
		assert_eq!(part_1(get_input_file("inputs/day_01/input.txt")), 1521);
	}

	#[test]
	fn test_part_2() {
		assert_eq!(part_2(get_input_file("inputs/day_01/input.txt")), 1543);
	}
}
