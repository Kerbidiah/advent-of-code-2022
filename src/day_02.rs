#![allow(dead_code)]

use std::fs;
use std::path::PathBuf;

use rayon::prelude::*;

#[derive(Debug)] // I do this for all enums and structs because there is no reason not to
#[derive(PartialEq, Eq, Clone, Copy)] // might need these later
enum Hand {
	Rock,
	Paper,
	Scissors
}

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
	pub fn value(&self) -> u32 {
		match self {
			Hand::Rock => 1,
			Hand::Paper => 2,
			Hand::Scissors => 3,
		}
	}
	
	// 0 if you lost, 3 if the round was a draw, and 6 if you won
	// you are the left side (instance method is called on)
	pub fn outcome(&self, other: &Self) -> u32 {
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
	pub fn score(&self, other: &Self) -> u32 {
		self.outcome(other) + self.value()
	}

	// you are .1, NOT .0
	pub fn score_pair(p: &Pair) -> u32 {
		p.1.score(&p.0)
	}

	pub fn load(path: PathBuf) -> Vec<Pair> {
		let contents = fs::read_to_string(path).unwrap();
		let mut lines: Vec<&str> = contents.split('\n').collect();
		lines.retain(|l| !l.is_empty());

		lines.par_iter().map(|s| Self::new_pair(s)).collect()
	}
}


#[derive(Debug)]
enum Outcome {
	Win,
	Draw,
	Lose
}

impl Outcome {
	// X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win
	pub fn from_char(c: char) -> Self {
		match c.to_ascii_lowercase() {
			'x' => Self::Lose,
			'y' => Self::Draw,
			'z' => Self::Win,
			_ => panic!()
		}
	}

	pub fn new_thingy(s: &str) -> (Hand, Self) {
		let arr: Vec<char> = s.chars().collect();

		(Hand::from_char(arr[0]), Self::from_char(arr[2]))
	}

	pub fn load(path: PathBuf) -> Vec<(Hand, Self)> {
		let contents = fs::read_to_string(path).unwrap();
		let mut lines: Vec<&str> = contents.split('\n').collect();
		lines.retain(|l| !l.is_empty());

		lines.par_iter().map(|s| Self::new_thingy(s)).collect()
	}

	pub fn get_move(&self, hand: Hand) -> Hand {
		match (self, hand) {
			(Outcome::Win, Hand::Rock) => Hand::Paper,
			(Outcome::Win, Hand::Paper) => Hand::Scissors,
			(Outcome::Win, Hand::Scissors) => Hand::Rock,
			(Outcome::Draw, Hand::Rock) => Hand::Rock,
			(Outcome::Draw, Hand::Paper) => Hand::Paper,
			(Outcome::Draw, Hand::Scissors) => Hand::Scissors,
			(Outcome::Lose, Hand::Rock) => Hand::Scissors,
			(Outcome::Lose, Hand::Paper) => Hand::Rock,
			(Outcome::Lose, Hand::Scissors) => Hand::Paper,
		}
	}

	pub fn make_pair(thingy: &(Hand, Self)) -> Pair {
		(thingy.0, thingy.1.get_move(thingy.0))
	}
}

fn solve_part_1(path: PathBuf) -> u32 {
	let pairs = Hand::load(path);

	pairs
		.par_iter()
		.map(Hand::score_pair)
		.sum()
}

fn solve_part_2(path: PathBuf) -> u32 {
	let thingy = Outcome::load(path);

	thingy
		.par_iter()
		.map(Outcome::make_pair)
		.map(|p| Hand::score_pair(&p))
		.sum()
}


#[cfg(test)]
mod test {
	use super::*;

	const EXAMPLE_PATH: &str = "inputs/day_02_exmp.txt";
	const REAL_PATH: &str = "inputs/day_02_real.txt";

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

	#[test]
	fn example_part_2() {
		let path = PathBuf::from(EXAMPLE_PATH);

		assert_eq!(12, solve_part_2(path));
	}

	#[test]
	fn real_part_2() {
		let path = PathBuf::from(REAL_PATH);

		assert_eq!(10274, solve_part_2(path));
	}

}
