#![allow(dead_code)]

use std::path::PathBuf;
use std::ops::RangeInclusive;

use rayon::prelude::*;

use crate::tools;

#[derive(Debug)]
struct Pair {
	pub a: RangeInclusive<u32>,
	pub b: RangeInclusive<u32>,
}

impl Pair {
	fn range_from_str(s: &str) -> RangeInclusive<u32> {
		let (first, second) = s.split_once('-').unwrap();
		let start = first.parse().unwrap();
		let end = second.parse().unwrap();

		RangeInclusive::new(start, end)
	}

	pub fn new(s: &String) -> Self {
		let (first, second) = s.split_once(',').unwrap();

		Self {
			a: Self::range_from_str(first),
			b: Self::range_from_str(second)
		}
	}

	pub fn parse(path: PathBuf) -> Vec<Self> {
		let lines = tools::read_lines(path, true);

		lines.par_iter().map(Self::new).collect()
	}

	fn is_a_in_b(&self) -> bool {
		self.b.contains(self.a.start()) && self.b.contains(self.a.end())
	}

	fn is_b_in_a(&self) -> bool {
		self.a.contains(self.b.start()) && self.a.contains(self.b.end())
	}

	pub fn is_fully_contained(&self) -> bool {
		self.is_a_in_b() || self.is_b_in_a()
	}

	pub fn is_overlaping(&self) -> bool {
		(
			(self.a.end() >= self.b.start())
			&&
			(self.a.start() <= self.b.end())
		) || (
			(self.b.end() >= self.a.start())
			&&
			(self.b.start() <= self.a.end())
		)
	}
}

fn solve_part_1(path: PathBuf) -> usize {
	let pairs = Pair::parse(path);

	pairs.par_iter().filter(|p| p.is_fully_contained()).count()
}

fn solve_part_2(path: PathBuf) -> usize {
	let pairs = Pair::parse(path);

	pairs.par_iter().filter(|p| p.is_overlaping()).count()
}


#[cfg(test)]
mod test {
	use super::*;

	const EXAMPLE_PATH: &str = "inputs/day_04_exmp.txt";
	const REAL_PATH: &str = "inputs/day_04_real.txt";

	#[test]
	fn example_part_1() {
		let path = PathBuf::from(EXAMPLE_PATH);

		assert_eq!(2, solve_part_1(path));
	}

	#[test]
	fn real_part_1() {
		let path = PathBuf::from(REAL_PATH);

		assert_eq!(595, solve_part_1(path));
	}

	#[test]
	fn example_part_2() {
		let path = PathBuf::from(EXAMPLE_PATH);

		assert_eq!(4, solve_part_2(path));
	}

	#[test]
	fn real_part_2() {
		let path = PathBuf::from(REAL_PATH);

		assert_eq!(952, solve_part_2(path));
	}
}
