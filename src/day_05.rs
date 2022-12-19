#![allow(dead_code)]

use std::path::PathBuf;

use crate::tools;

mod ship;
mod command;

use ship::Ship;
use command::Command;

fn load(path: PathBuf) -> (Ship, Vec<Command>) {
	let mut lines = tools::read_lines(path, false);

	// find the index of the split between the ship and the commands
	let mut i = 0;
	while !lines[i].is_empty() {
		i += 1;
	}

	// split and get the commands
	let cmds = lines.split_off(i + 1);
	let commands = Command::new_command_list(&cmds);

	// get the ship
	lines.pop(); // delete the blank line
	let ship = Ship::new(&mut lines);

	(ship, commands)
}

pub fn solve_part_1(path: PathBuf) -> String {
	let (mut ship, cmds) = load(path);

	ship.run(&cmds);
	ship.tops()
}

pub fn solve_part_2(path: PathBuf) -> String {
	let (mut ship, cmds) = load(path);

	ship.run(&cmds);
	ship.tops()
}


#[cfg(test)]
mod test {
	use super::*;

	const EXAMPLE_PATH: &str = "inputs/day_05_exmp.txt";
	const REAL_PATH: &str = "inputs/day_05_real.txt";

	#[test]
	fn example_part_1() {
		let path = PathBuf::from(EXAMPLE_PATH);

		assert_eq!("CMZ", solve_part_1(path));
	}

	#[test]
	fn real_part_1() {
		let path = PathBuf::from(REAL_PATH);

		assert_eq!("JDTMRWCQJ", solve_part_1(path));
	}

	#[test]
	fn example_part_2() {
		let path = PathBuf::from(EXAMPLE_PATH);

		assert_eq!("MCD", solve_part_2(path));
	}

	#[test]
	fn real_part_2() {
		let path = PathBuf::from(REAL_PATH);

		assert_eq!("00000000000", solve_part_2(path));
	}
}
