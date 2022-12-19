#![allow(dead_code)]

use std::path::PathBuf;

use crate::tools;

mod filesystem;

fn solve_part_1(path: PathBuf) -> u32 {
	let lines = tools::read_lines(path, true);

	todo!()
}


#[cfg(test)]
mod test {
	use super::*;

	const EXAMPLE_PATH: &str = "inputs/day_07_exmp.txt";
	const REAL_PATH: &str = "inputs/day_07_real.txt";

	#[test]
	fn example_part_1() {
		let path = PathBuf::from(EXAMPLE_PATH);

		assert_eq!(95437, solve_part_1(path));
	}

	#[test]
	fn real_part_1() {
		let path = PathBuf::from(REAL_PATH);

		assert_eq!(00000000000, solve_part_1(path));
	}

	// #[test]
	// fn example_part_2() {
	// 	let path = PathBuf::from(EXAMPLE_PATH);

	// 	assert_eq!(00000000000, solve_part_2(path));
	// }

	// #[test]
	// fn real_part_2() {
	// 	let path = PathBuf::from(REAL_PATH);

	// 	assert_eq!(00000000000, solve_part_2(path));
	// }
}
