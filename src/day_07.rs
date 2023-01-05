#![allow(dead_code)]

use std::path::PathBuf;

use rayon::prelude::*;

mod filesystem;
mod parse;

fn solve_part_1(path: PathBuf) -> u64 {
	let mut filesys = parse::parse(path);

	let _ = filesys.size(0); // calculate the size of everything

	filesys.contents.par_iter()
		.filter(|obj| obj.is_folder())
		.filter(|obj| obj.size.unwrap() < 100000)
		.map(|obj| obj.size.unwrap())
		.sum()
}

fn solve_part_2(path: PathBuf) -> u64 {
	let mut filesys = parse::parse(path);
	
	let space = 70000000;
	let needed_space = 30000000;
	let goal = space - needed_space;

	let total_space = filesys.size(0); // calculate the size of everything
	let reduction = total_space - goal;

	filesys.contents.par_iter()
		.filter(|obj| obj.is_folder())
		.filter(|obj| obj.size.unwrap() >= reduction) // keep directories that are big enough
		.map(|obj| obj.size.unwrap())
		.min().unwrap()
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

		assert_eq!(1648397, solve_part_1(path));
	}

	#[test]
	fn example_part_2() {
		let path = PathBuf::from(EXAMPLE_PATH);

		assert_eq!(24933642, solve_part_2(path));
	}

	#[test]
	fn real_part_2() {
		let path = PathBuf::from(REAL_PATH);

		assert_eq!(1815525, solve_part_2(path));
	}
}
