#![allow(dead_code)]

use std::path::PathBuf;
use std::fs;

fn load_callories(path: PathBuf) -> Vec<u32> {
	let contents = fs::read_to_string(path).unwrap();
	let lines = contents.split('\n');

	let mut calories = Vec::new();
	let mut count: u32 = 0;
	for each in lines {
		if !each.is_empty() {
			count += each.parse::<u32>().unwrap(); // convert string to u32
		} else { // if the line is empty, add curent elf and reset for next elf
			calories.push(count);
			count = 0;
		}
	}
	calories.push(count); // make sure last elf's callories get appended

	calories
}

pub fn solve_part_1(path: PathBuf) -> u32 {
	let arr = load_callories(path);

	*arr.iter().max().unwrap() // * dereferences the borrow (this can be done becuase u32 implements copy)
}

pub fn solve_part_2(path: PathBuf) -> u32 {
	let mut arr = load_callories(path);
	let i = arr.len() - 1;
	arr.sort();


	arr[i - 2] + arr[i - 1] + arr[i]
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn example_part_1() {
		let path = PathBuf::from("inputs/day_01_exmp.txt");

		assert_eq!(24000, solve_part_1(path));
	}

	#[test]
	fn real_part_1() {
		let path = PathBuf::from("inputs/day_01_real.txt");

		assert_eq!(70374, solve_part_1(path));
	}

	#[test]
	fn example_part_2() {
		let path = PathBuf::from("inputs/day_01_exmp.txt");

		assert_eq!(45000, solve_part_2(path));
	}

	#[test]
	fn real_part_2() {
		let path = PathBuf::from("inputs/day_01_real.txt");

		assert_eq!(204610, solve_part_2(path));
	}

}