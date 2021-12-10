use std::fs;

fn get_input_file(path: &str) -> Vec<usize> {
	let contents = fs::read_to_string(path).expect("Unable to read input file");

	return contents
		.split(",")
		.map(|s: &str| s.parse::<usize>().unwrap())
		.collect();
}

fn part_1(fish: Vec<usize>, days: usize) -> usize {
	let mut fish_groups = vec![0; 9];

	for val in fish {
		fish_groups[val] += 1;
	}

	for _ in 0..days {
		let new_fish = fish_groups.remove(0);

		fish_groups[6] += new_fish;
		fish_groups.push(new_fish);
	}

	return fish_groups.iter().sum();
}

pub fn main() {
	println!(
		"Day 6.1: {}",
		part_1(get_input_file("inputs/day_06/input.txt"), 80)
	);

	println!(
		"Day 6.2: {}",
		part_1(get_input_file("inputs/day_06/input.txt"), 256)
	);
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_part_1() {
		assert_eq!(
			part_1(get_input_file("inputs/day_06/input.txt"), 80),
			386536
		);
	}

	#[test]
	fn test_part_2() {
		assert_eq!(part_1(get_input_file("inputs/day_06/sample.txt"), 256), 12);
	}
}
