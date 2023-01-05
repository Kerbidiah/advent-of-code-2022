#![allow(dead_code)]

use std::path::PathBuf;

use rayon::prelude::*;

use crate::tools;

mod vmap;

fn parse(path: PathBuf) -> Vec<Vec<u8>> {
	let lines = tools::read_lines(path, true);

	lines.par_iter().map(|line| {
		line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect()
	}).collect()
}

fn solve_part_1(path: PathBuf) -> usize {
	let tree_map = parse(path);
	let mut vis_map = vmap::VMap::new(&tree_map);

	vis_map.check_visible(&tree_map);

	vis_map.count_visible()
}

fn solve_part_2(path: PathBuf) -> usize {
	let tree_map = parse(path);
	let mut vis_map = vmap::VMap::new(&tree_map);

	vis_map.check_visible(&tree_map);

	vis_map.count_visible()
}


#[cfg(test)]
mod test {
	use super::*;

	const EXAMPLE_PATH: &str = "inputs/day_08_exmp.txt";
	const REAL_PATH: &str = "inputs/day_08_real.txt";

	#[test]
	fn example_part_1() {
		let path = PathBuf::from(EXAMPLE_PATH);

		assert_eq!(21, solve_part_1(path));
	}

	#[test]
	fn real_part_1() {
		let path = PathBuf::from(REAL_PATH);

		assert_eq!(1825, solve_part_1(path));
	}

	#[test]
	fn example_part_2() {
		let path = PathBuf::from(EXAMPLE_PATH);

		assert_eq!(00000000000, solve_part_2(path));
	}

	#[test]
	fn real_part_2() {
		let path = PathBuf::from(REAL_PATH);

		assert_eq!(00000000000, solve_part_2(path));
	}
}
