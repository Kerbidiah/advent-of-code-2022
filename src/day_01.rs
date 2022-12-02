#![allow(dead_code)]

use std::path::PathBuf;
use std::fs;

pub fn solve_part_1(path: PathBuf) -> u32 {
	let contents = fs::read_to_string(path).unwrap();
	let lines = contents.split('\n');

	let mut arr = Vec::new();
	let mut count: u32 = 0;
	for each in lines {
		if !each.is_empty() {
			count += each.parse::<u32>().unwrap(); // convert string to u32
		} else { // if the line is empty, add curent elf and reset for next elf
			arr.push(count);
			count = 0;
		}
	}
	arr.push(count); // make sure last elf's callories get appended

	*arr.iter().max().unwrap() // * dereferences the borrow (this can be done becuase u32 implements copy)
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn example_part_1() {
		let path = PathBuf::from("inputs/day_01_exmp");

		assert_eq!(24000, solve_part_1(path));
	}

	#[test]
	fn real_part_1() {
		let path = PathBuf::from("inputs/day_01_real");

		assert_eq!(70374, solve_part_1(path));
	}
}