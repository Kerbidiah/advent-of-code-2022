#![allow(dead_code)]

use std::fs;
use std::path::PathBuf;

fn solve_part_1(path: PathBuf) -> u32 {
	todo!()
}


#[cfg(test)]
mod test {
	use super::*;

	const EXAMPLE_PATH: String = "inputs/day_02_exmp";
	const REAL_PATH: String = "inputs/day_02_real";

	#[test]
	fn example_part_1() {
		let path = PathBuf::from(EXAMPLE_PATH);

		assert_eq!(15, solve_part_1(path));
	}

	#[test]
	fn real_part_1() {
		let path = PathBuf::from(REAL_PATH);

		assert_eq!(14375, solve_part_1(path));
	}

	// #[test]
	// fn example_part_2() {
	// 	let path = PathBuf::from(EXAMPLE_PATH);

	// 	assert_eq!(12, solve_part_2(path));
	// }

	// #[test]
	// fn real_part_2() {
	// 	let path = PathBuf::from(REAL_PATH);

	// 	assert_eq!(10274, solve_part_2(path));
	// }
}
