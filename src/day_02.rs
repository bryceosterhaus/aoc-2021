use std::fs;

fn get_input_file(path: &str) -> Vec<String> {
	let contents = fs::read_to_string(path).expect("Unable to read input file");

	return contents.lines().map(|s: &str| s.to_string()).collect();
}

fn part_1(input: &Vec<String>) -> u32 {
	let mut x = 0;
	let mut y = 0;

	for line in input {
		let command = line.split_whitespace().collect::<Vec<_>>();
		let value = command[1].parse::<u32>().unwrap();

		match command[0] {
			"forward" => x += value,
			"down" => y += value,
			"up" => y -= value,
			_ => panic!("i broke"),
		}
	}

	return x * y;
}

fn part_2(input: &Vec<String>) -> u32 {
	let mut x = 0;
	let mut y = 0;
	let mut aim = 0;

	for line in input {
		let command = line.split_whitespace().collect::<Vec<_>>();
		let value = command[1].parse::<u32>().unwrap();

		match command[0] {
			"forward" => {
				x += value;
				y += aim * value;
			}
			"down" => aim += value,
			"up" => aim -= value,
			_ => panic!("i broke"),
		}
	}

	return x * y;
}

pub fn main() {
	println!(
		"Distance: {}",
		part_1(&get_input_file("inputs/day_02/input.txt"))
	);

	println!(
		"Distance: {}",
		part_2(&get_input_file("inputs/day_02/input.txt"))
	);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part_1() {
		assert_eq!(part_1(&get_input_file("inputs/day_02/input.txt")), 1654760);
	}

	#[test]
	fn test_part_2() {
		assert_eq!(
			part_2(&get_input_file("inputs/day_02/input.txt")),
			1956047400
		);
	}
}
