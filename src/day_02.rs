#![allow(dead_code)]

use std::fs;
use std::path::PathBuf;

use rayon::prelude::*;

#[derive(Debug)] // I do this for all enums and structs because there is no reason not to
#[derive(PartialEq, Eq)] // might need this later
enum Hand {
	Rock,
	Paper,
	Scissors
}

type Num = u32; // I can easily change the type we are using for all numbers if I want to
type Pair = (Hand, Hand);

impl Hand {

	pub fn new_pair(s: &str) -> Pair {
		let thingy: Vec<char> = s.chars().collect();

		(Self::from_char(thingy[0]), Self::from_char(thingy[2]))
	}

	// A for Rock, B for Paper, and C for Scissors
	// X for Rock, Y for Paper, and Z for Scissors
	pub fn from_char(c: char) -> Self {
		match c.to_ascii_lowercase() {
			'a' | 'x' => Self::Rock,
			'b' | 'y' => Self::Paper,
			'c' | 'z' => Self::Scissors,
			_ => panic!()
		}
	}
	
	// 1 for Rock, 2 for Paper, and 3 for Scissors
	pub fn value(&self) -> Num {
		match self {
			Hand::Rock => 1,
			Hand::Paper => 2,
			Hand::Scissors => 3,
		}
	}
	
	// 0 if you lost, 3 if the round was a draw, and 6 if you won
	// you are the left side (instance method is called on)
	pub fn outcome(&self, other: &Self) -> Num {
		match (self, other) {
			(Hand::Rock, Hand::Rock) => 3,
			(Hand::Rock, Hand::Paper) => 0,
			(Hand::Rock, Hand::Scissors) => 6,
			(Hand::Paper, Hand::Rock) => 6,
			(Hand::Paper, Hand::Paper) => 3,
			(Hand::Paper, Hand::Scissors) => 0,
			(Hand::Scissors, Hand::Rock) => 0,
			(Hand::Scissors, Hand::Paper) => 6,
			(Hand::Scissors, Hand::Scissors) => 3,
		}
	}

	// The score for a single round is the score for the shape you selected
	// plus the score for the outcome of the round
	pub fn score(&self, other: &Self) -> Num {
		self.outcome(other) + self.value()
	}

	// you are .1, NOT .0
	pub fn score_pair(p: &Pair) -> Num {
		p.1.score(&p.0)
	}

	pub fn load(path: PathBuf) -> Vec<Pair> {
		let contents = fs::read_to_string(path).unwrap();
		let mut lines: Vec<&str> = contents.split('\n').collect();
		lines.retain(|l| !l.is_empty());

		lines.par_iter().map(|s| Self::new_pair(s)).collect()
	}
}

fn solve_part_1(path: PathBuf) -> Num {
	let pairs = Hand::load(path);

	pairs
		.iter() // TOOD: convert back to par_iter
		.map(|p| Hand::score_pair(p))
		.sum()
}


#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn example_part_1() {
		let path = PathBuf::from("inputs/day_02_exmp");

		assert_eq!(15, solve_part_1(path));
	}

	#[test]
	fn real_part_1() {
		let path = PathBuf::from("inputs/day_02_real");

		assert_eq!(14375, solve_part_1(path));
	}

	// #[test]
	// fn example_part_2() {
	// 	let path = PathBuf::from("inputs/day_02_exmp");

	// 	assert_eq!(12, solve_part_2(path));
	// }

	// #[test]
	// fn real_part_2() {
	// 	let path = PathBuf::from("inputs/day_02_real");

	// 	assert_eq!(15, solve_part_2(path));
	// }

}
