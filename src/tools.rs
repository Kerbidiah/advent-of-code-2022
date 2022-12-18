use std::fs;
use std::path::PathBuf;

pub fn read_lines(path: PathBuf, del_blank_lines: bool) -> Vec<String> {
	let contents = fs::read_to_string(path).unwrap();
	let mut lines: Vec<&str> = contents.split("\n").collect();
	
	if del_blank_lines {
		lines.retain(|line| !line.is_empty());
	}

	lines.iter().map(|l| l.to_string()).collect()
}