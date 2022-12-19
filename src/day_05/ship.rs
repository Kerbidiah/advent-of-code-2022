use itertools::Itertools;
use rayon::prelude::{IntoParallelRefMutIterator, ParallelIterator};

use super::command::Command;

#[derive(Debug)]
pub struct Ship {
	stacks: Vec<Vec<char>>
}

impl Ship {
	fn fill_column(arr: &Vec<String>, i: usize) -> Vec<char> {
		let k = i - 1;

		arr.iter()
			.map(|x| x.as_bytes()[k] as char)
			.filter(|x| x != &' ') // filter out the spaces
			.rev()
			.collect()
	}

	pub fn new(arr: &mut Vec<String>) -> Self {
		// get the index row and find how many stacks we have
		let index_row = arr.pop().unwrap();
		let max_index = index_row
			.split(' ')
			.filter(|x| !x.is_empty())
			.last().unwrap()
			.parse().unwrap();
		
		// create the stacks and pre-allocate enough space so it shouldn't
		// need to allocate later
		let size = arr.len() * max_index * 2 / 3;
		let mut stacks = vec![Vec::with_capacity(size); max_index];

		// drop the first and last char
		arr.par_iter_mut().for_each(|l| {
			l.pop().unwrap();
			l.remove(0);
		});

		// remove the brakets and spaces
		let mut i = 1;
		while i < max_index {
			arr.iter_mut().for_each(|l| {
				l.remove(i);
				l.remove(i);
				l.remove(i);
			});

			// fill current column in stacks
			stacks[i - 1] = Self::fill_column(arr, i);
			
			i += 1;
		}

		stacks[i - 1] = Self::fill_column(arr, i);

		Self {
			stacks
		}
	}

	pub fn execute_command(&mut self, command: &Command) {
		let mut i = 0;
		while i < command.qty {
			let item = self.stacks[command.from].pop().unwrap();
			self.stacks[command.to].push(item);

			i += 1;
		}
	}

	pub fn run(&mut self, commands: &[Command]) {
		commands.iter().for_each(|c| {
			self.execute_command(c);
		});
	}

	pub fn tops(&self) -> String {
		self.stacks.iter().map(|stack| stack.last().unwrap()).join("")
	}
}