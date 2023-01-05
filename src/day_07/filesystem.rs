use rayon::prelude::*;

#[derive(Debug)]
pub struct Storage {
	pub contents: Vec<FileObject>,
	pub locked: bool
}

#[derive(Debug)]
pub enum Contents {
	Folder(Vec<ID>),
	File
}

pub type ID = usize;
const BLANK_ID: ID = usize::MAX; // id to signify a file that hasn't yet been stored

#[derive(Debug)]
pub struct FileObject {
	pub name: String,
	pub id: ID,
	pub size: Option<u64>,
	pub contents: Contents,
}

impl Storage {
	fn assert_unlocked(&self) {
		if self.locked {
			panic!("STORAGE IS LOCKED!!!\nreading not allowed")
		}
	}

	pub fn new(name: String) -> Self {
		let file = FileObject {
			name,
			id: 0,
			size: None,
			contents: Contents::Folder(Vec::new()),
		};

		Self {
			contents: vec![file],
			locked: false,
		}
	}

	fn store(&mut self, mut obj: FileObject) -> ID {
		self.assert_unlocked();
		let id = self.contents.len();

		obj.id = id; // set correct ID
		self.contents.push(obj); // add it to storage

		id
	}

	pub fn add(&mut self, obj: FileObject, parent_id: ID) -> ID {
		let id = self.store(obj);

		match &mut self.contents[parent_id].contents {
			Contents::File => panic!("can't add since this is a file"),
			Contents::Folder(arr) => arr.push(id),
		}

		id
	}

	pub fn size(&mut self, obj_id: ID) -> u64 {
		self.locked = true;

		if let Some(size) = &self.contents[obj_id].size {
			// if size already known
			*size
		} else {
			let id_arr = match &self.contents[obj_id].contents {
				Contents::Folder(x) => x.to_owned(),
				Contents::File => unreachable!(),
			};

			let new_size = &id_arr.iter().map(|id| {
					self.size(*id)
				}).sum();

			self.contents[obj_id].size = Some(*new_size);

			*new_size
		}
	}

	pub fn search(&self, obj_id: ID, name: &str) -> ID {
		let file = &self.contents[obj_id];
		match &file.contents {
			Contents::File => {
				if file.is_name(name) {
					self.contents[obj_id].id
				} else {
					panic!("NO MATCH (in file)")
				}
			},
			Contents::Folder(id_arr) => {
				for &id in id_arr {
					if self.contents[id].is_name(name) {
						return id
					}
				}

				panic!("NO MATCH (in folder)")
			},
		}
	}

}

impl FileObject {
	pub fn new_file(name: &str, size: u64) -> Self {
		Self {
			name: name.to_string(),
			id: BLANK_ID,
			size: Some(size),
			contents: Contents::File
		}
	}

	pub fn new_folder(name: &str) -> Self {
		Self {
			name: name.to_string(),
			id: BLANK_ID,
			size: None,
			contents: Contents::Folder(vec![])
		}
	}

	pub fn is_name(&self, name: &str) -> bool {
		&self.name == name
	}

	pub fn is_folder(&self) -> bool {
		match self.contents {
			Contents::File => false,
			Contents::Folder(_) => true,
		}
	}
}
