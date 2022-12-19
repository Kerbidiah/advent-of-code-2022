#![allow(dead_code)]

use std::path::PathBuf;

use crate::tools;

fn solve_part_1(path: PathBuf) -> u32 {
	todo!()
}


#[cfg(test)]
mod test {
	use super::*;

	const EXAMPLE_PATH: [&str; 5] = [
		"inputs/day_02_exmp_0.txt",
		"inputs/day_02_exmp_1.txt",
		"inputs/day_02_exmp_2.txt",
		"inputs/day_02_exmp_3.txt",
		"inputs/day_02_exmp_4.txt",
	];

	const REAL_PATH: &str = "inputs/day_02_real.txt";

	#[test]
	fn example_part_1() {
		let ans = vec![7, 5, 6, 10, 11];

		let mut i = 0;
		for each in EXAMPLE_PATH {
			let path = PathBuf::from(each);
			assert_eq!(ans[i], solve_part_1(path));
			i += 1;
		}

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
