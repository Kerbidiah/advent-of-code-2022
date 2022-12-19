#![allow(dead_code)]

use std::path::PathBuf;
use std::collections::HashSet;

use crate::tools;

fn solve_part_1(path: PathBuf) -> usize {
	let everything = tools::read_lines(path, true);
	let line = everything[0].as_bytes();

	let mut k = 0;
	let mut i = 4;
	let mut set: HashSet<&u8> = line[k..i].into_iter().collect();

	while set.len() != 4 {
		set.clear();
		set = line[k..i].into_iter().collect();

		i += 1;
		k += 1;
	}

	i - 1
}

fn solve_part_2(path: PathBuf) -> usize {
	let everything = tools::read_lines(path, true);
	let line = everything[0].as_bytes();

	let mut k = 0;
	let mut i = 14;
	let mut set: HashSet<&u8> = line[k..i].into_iter().collect();

	while set.len() != 14 {
		set.clear();
		set = line[k..i].into_iter().collect();

		i += 1;
		k += 1;
	}

	i - 1
}



#[cfg(test)]
mod test {
	use super::*;

	const ANS_PART_1: [usize; 5] = [7, 5, 6, 10, 11];
	const ANS_PART_2: [usize; 5] = [19, 23, 23, 29, 26];

	const EXAMPLE_PATH: [&str; 5] = [
		"inputs/day_06_exmp_0.txt",
		"inputs/day_06_exmp_1.txt",
		"inputs/day_06_exmp_2.txt",
		"inputs/day_06_exmp_3.txt",
		"inputs/day_06_exmp_4.txt",
	];

	const REAL_PATH: &str = "inputs/day_06_real.txt";

	#[test]
	fn example_part_1_num_0() {
		let i: usize = 0;

		let path = PathBuf::from(EXAMPLE_PATH[i]);

		assert_eq!(ANS_PART_1[i], solve_part_1(path))
	}

	#[test]
	fn example_part_1_num_1() {
		let i: usize = 1;

		let path = PathBuf::from(EXAMPLE_PATH[i]);

		assert_eq!(ANS_PART_1[i], solve_part_1(path))
	}

	#[test]
	fn example_part_1_num_2() {
		let i: usize = 2;

		let path = PathBuf::from(EXAMPLE_PATH[i]);

		assert_eq!(ANS_PART_1[i], solve_part_1(path))
	}

	#[test]
	fn example_part_1_num_3() {
		let i: usize = 3;

		let path = PathBuf::from(EXAMPLE_PATH[i]);

		assert_eq!(ANS_PART_1[i], solve_part_1(path))
	}

	#[test]
	fn example_part_1_num_4() {
		let i: usize = 4;

		let path = PathBuf::from(EXAMPLE_PATH[i]);

		assert_eq!(ANS_PART_1[i], solve_part_1(path))
	}

	#[test]
	fn real_part_1() {
		let path = PathBuf::from(REAL_PATH);

		assert_eq!(1531, solve_part_1(path));
	}

	#[test]
	fn example_part_2_num_0() {
		let i: usize = 0;

		let path = PathBuf::from(EXAMPLE_PATH[i]);

		assert_eq!(ANS_PART_2[i], solve_part_2(path))
	}

	#[test]
	fn example_part_2_num_1() {
		let i: usize = 1;

		let path = PathBuf::from(EXAMPLE_PATH[i]);

		assert_eq!(ANS_PART_2[i], solve_part_2(path))
	}

	#[test]
	fn example_part_2_num_2() {
		let i: usize = 2;

		let path = PathBuf::from(EXAMPLE_PATH[i]);

		assert_eq!(ANS_PART_2[i], solve_part_2(path))
	}

	#[test]
	fn example_part_2_num_3() {
		let i: usize = 3;

		let path = PathBuf::from(EXAMPLE_PATH[i]);

		assert_eq!(ANS_PART_2[i], solve_part_2(path))
	}

	#[test]
	fn example_part_2_num_4() {
		let i: usize = 4;

		let path = PathBuf::from(EXAMPLE_PATH[i]);

		assert_eq!(ANS_PART_2[i], solve_part_2(path))
	}

	#[test]
	fn real_part_2() {
		let path = PathBuf::from(REAL_PATH);

		assert_eq!(2518, solve_part_2(path));
	}
}
