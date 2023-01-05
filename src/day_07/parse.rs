use std::path::PathBuf;

use crate::tools;
use super::filesystem::*;

pub fn parse(path: PathBuf) -> Storage {
	// read file and setup iterator
	let thingy = tools::read_lines(path, true);
	let lines = thingy.iter();
	
	let mut storage = Storage::new("/".to_string());
	let mut path = vec![0];

	for line in lines {
		let cmd: Vec<&str> = line.split(' ').collect();
		match cmd[0] {
			"$" => { // command
				command(&cmd, &mut storage, &mut path);
			},
			"dir" => { // new folder
				let obj = FileObject::new_folder(cmd[1]);
				storage.add(obj, *path.last().unwrap());
			},
			_ => { // new file
				let obj = FileObject::new_file(cmd[1], cmd[0].parse().unwrap());
				storage.add(obj, *path.last().unwrap());
			}
		}
	}

	storage
}

fn command(cmd: &Vec<&str>, filesys: &mut Storage, path: &mut Vec<ID>) {
	match cmd[1] {
		"ls" => { // list files
			// do nothing
		},
		"cd" => { // change path
			match cmd[2] {
				"/" => *path = vec![0],
				".." => drop(path.pop().unwrap()),
				x => {
					let new_id = filesys.search(*path.last().unwrap(), x);
					path.push(new_id)
				},
			}
		},
		_ => panic!("unknown commad")
	}
}
