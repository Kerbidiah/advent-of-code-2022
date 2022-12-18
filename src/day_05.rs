#![allow(dead_code)]

use std::path::PathBuf;

use crate::tools;

fn solve_part_1(path: PathBuf) -> u32 {
	todo!()
}


#[cfg(test)]
mod test {
	use super::*;

	const EXAMPLE_PATH: &str = "inputs/day_05_exmp.txt";
	const REAL_PATH: &str = "inputs/day_05_real.txt";

	#[test]
	fn example_part_1() {
		let path = PathBuf::from(EXAMPLE_PATH);

		assert_eq!(00000000000, solve_part_1(path));
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
