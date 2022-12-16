#![allow(dead_code)]

use std::fs;
use std::path::PathBuf;
use std::collections::HashSet;

use itertools::Itertools;

use rayon::prelude::*;

fn str_to_hashset(s: &str) -> HashSet<char> {
	let mut set = HashSet::with_capacity(s.len());

	for c in s.chars() {
		set.insert(c);
	}

	set
}

fn score_char(c: char) -> u8 {
	// Lowercase item types a through z have priorities 1 through 26.
    // Uppercase item types A through Z have priorities 27 through 52.

	let byte = u8::try_from(c).unwrap();

	if c.is_ascii_uppercase() {
		// A = 65 in ascii
		// byte - 64 + 26 = byte - 38
		byte - 38
	} else if c.is_ascii_lowercase() {
		// a = 97 in ascii
		byte - 96
	} else {
		panic!()
	}
}

type HashSetPair = (HashSet<char>, HashSet<char>);

fn load(path: PathBuf) -> Vec<HashSetPair> {
	let contents = fs::read_to_string(path).unwrap();
	let mut lines: Vec<&str> = contents.split("\n").collect();
	lines.retain(|line| !line.is_empty());

	lines
		.par_iter()
		.map(|line| {
			let (first, last) = line.split_at(line.len()/2);

			(str_to_hashset(first), str_to_hashset(last))
		}).collect()
}

fn get_unions(arr: Vec<HashSetPair>) -> Vec<char> {
	arr
		.par_iter()
		.map(|set| {
			*set.0.intersection(&set.1).next().unwrap()
		}).collect()
}

fn solve_part_1(path: PathBuf) -> u32 {
	let sets_arr = load(path);

	get_unions(sets_arr)
		.par_iter()
		.map(|x| {
			score_char(*x) as u32
		}).sum()
}

fn solve_part_2(path: PathBuf) -> u32 {

	// load lines
	let contents = fs::read_to_string(path).unwrap();
	let mut lines: Vec<&str> = contents.split("\n").collect();
	lines.retain(|line| !line.is_empty());

	// create groups
	let groups: Vec<Vec<HashSet<char>>> = lines
		.iter().chunks(3).into_iter()
		.map(|chunk| {
			chunk.map(|line| {
				str_to_hashset(line)
			}).collect::<Vec<HashSet<char>>>()
		}).collect();
	
	// find the badges for each group and then sum them
	groups
		.iter()
		.map(|group| {
			let union_a: HashSet<_> = group[0].intersection(&group[1]).map(|x| x.to_owned()).collect();

			let mut set = group[2].intersection(&union_a);
			dbg!(&set);
			
			let c = set.next().unwrap().to_owned();

			score_char(c) as u32
		}).sum()
}


#[cfg(test)]
mod test {
	use super::*;

	const EXAMPLE_PATH: &str = "inputs/day_03_exmp";
	const REAL_PATH: &str = "inputs/day_03_real";

	fn char_score_test_r() {
		assert_eq!(18, score_char('r'));
	}

	#[allow(non_snake_case)]
	fn char_score_test_Z() {
		assert_eq!(52, score_char('Z'));
	}

	#[test]
	fn example_part_1() {
		let path = PathBuf::from(EXAMPLE_PATH);

		assert_eq!(157, solve_part_1(path));
	}

	#[test]
	fn real_part_1() {
		let path = PathBuf::from(REAL_PATH);

		assert_eq!(8123, solve_part_1(path));
	}

	#[test]
	fn example_part_2() {
		let path = PathBuf::from(EXAMPLE_PATH);

		assert_eq!(70, solve_part_2(path));
	}

	#[test]
	fn real_part_2() {
		let path = PathBuf::from(REAL_PATH);

		assert_eq!(10274, solve_part_2(path));
	}
}
