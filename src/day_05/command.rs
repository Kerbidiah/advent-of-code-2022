use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug)]
pub struct Command {
	pub from: usize,
	pub to: usize,
	pub qty: u8,
}

impl Command {
	pub fn new(s: &String) -> Self {
		let start = s.strip_prefix("move ").unwrap();
		let (q, rem) = start.split_once(" from ").unwrap();
		let (f, t) = rem.split_once(" to ").unwrap();

		Self {
			from: f.parse::<usize>().unwrap() - 1,
			to: t.parse::<usize>().unwrap() - 1,
			qty: q.parse().unwrap()
		}
	}

	pub fn new_command_list(arr: &[String]) -> Vec<Self> {
		arr.par_iter().filter(|l| !l.is_empty()).map(Command::new).collect()
	}
}