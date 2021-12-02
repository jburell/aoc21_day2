use std::fs;
use aoc21_day2::*;

fn get_data(filename: &str) -> Vec<(String, u32)> {
  fs::read_to_string(filename).unwrap()
    .lines()
    .map(|s| s.trim().to_owned())
    .filter(|s| !s.is_empty())
    .map(|s| 
			s.split_once(' ')
			.map(|(cmd, value)| (cmd.to_owned(), u32::from_str_radix(value,10).unwrap()))
			.unwrap()
		)
    .collect()
}

#[test]
fn part_1() {
		// Arrange
		let commands = get_data("input_part1.txt");

		// Act
		let result = calc_dive_coords(commands);

		// Assert
		assert_eq!(true, result.is_ok());
		assert_eq!(1648020, result.unwrap());
}
