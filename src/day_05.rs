use std::collections::HashMap;
use std::fs;

fn get_input_file(path: &str) -> Vec<String> {
	let contents = fs::read_to_string(path).expect("Unable to read input file");

	return contents.lines().map(|s: &str| s.to_string()).collect();
}

fn part_1(vents: Vec<String>) -> u32 {
	let mut map = HashMap::new();

	// scores.insert(String::from("Blue"), 10);

	for vent in vents {
		let points = vent.split(" -> ").collect::<Vec<&str>>();

		let first_point = points[0]
			.split(",")
			.map(|x| x.parse::<i32>().unwrap())
			.collect::<Vec<i32>>();

		let last_point = points[1]
			.split(",")
			.map(|x| x.parse::<i32>().unwrap())
			.collect::<Vec<i32>>();

		let start: i32;
		let end: i32;

		// Horizontal Line
		if first_point[0] == last_point[0] {
			if first_point[1] < last_point[1] {
				start = first_point[1];
				end = last_point[1]
			} else {
				start = last_point[1];
				end = first_point[1]
			};
			for i in start..end + 1 {
				let coordinate = String::from(format!("{},{}", first_point[0], i));
				let count = map.entry(coordinate).or_insert(0);
				*count += 1;
			}
		// Vertical Line
		} else if first_point[1] == last_point[1] {
			if first_point[0] < last_point[0] {
				start = first_point[0];
				end = last_point[0]
			} else {
				start = last_point[0];
				end = first_point[0]
			};
			for i in start..end + 1 {
				let coordinate = String::from(format!("{},{}", i, first_point[1]));
				let count = map.entry(coordinate).or_insert(0);
				*count += 1;
			}
		}
	}

	let mut total = 0;

	for (_key, value) in map.iter() {
		if *value >= 2 {
			total += 1;
		}
	}

	return total;
}

pub fn main() {
	println!(
		"Day 5: {}",
		part_1(get_input_file("inputs/day_05/input.txt"))
	);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_sample_part_1() {
		assert_eq!(part_1(get_input_file("inputs/day_05/sample.txt")), 5);
	}

	#[test]
	fn test_part_1() {
		assert_eq!(part_1(get_input_file("inputs/day_05/input.txt")), 8622);
	}
}
