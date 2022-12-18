use std::fs;
use std::path::PathBuf;

pub fn read_lines(path: PathBuf) -> Vec<String> {
	let contents = fs::read_to_string(path).unwrap();
	let mut lines: Vec<&str> = contents.split("\n").collect();
	lines.retain(|line| !line.is_empty());

	lines.iter().map(|l| l.to_string()).collect()
}