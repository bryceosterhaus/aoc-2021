use std::fs;

fn text_input_to_vec(path: &str) -> Vec<u32> {
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

pub fn main() {
	println!(
		"Increments: {}",
		part_1(text_input_to_vec("inputs/day_01/part_1.txt"))
	);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part_1() {
		assert_eq!(part_1(text_input_to_vec("inputs/day_01/part_1.txt")), 1521);
	}
}
