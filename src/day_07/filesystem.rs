
#[derive(Debug)]
pub enum Contents {
	Folder(Vec<FileObject>),
	File(u64)
}

#[derive(Debug)]
pub struct FileObject {
	pub name: String,
	pub contents: Contents,
}

impl FileObject {
	pub fn add(&mut self, obj: Self) {
		match &mut self.contents {
			Contents::File(_) => panic!("can't add since this is a folder"),
			Contents::Folder(files) => {
				files.push(obj);
			}
		}
	}

	pub fn add_file(&mut self, name: String, size: u64) {
		let file = Self {
			name,
			contents: Contents::File(size)
		};

		self.add(file);
	}

	pub fn add_folder(&mut self, name: String, files: Option<Vec<Self>>) {
		let folder = Self {
			name,
			contents: Contents::Folder(files.unwrap_or_default())
		};

		self.add(folder);
	}

	pub fn size(&self) -> u64 {
		match &self.contents {
			Contents::File(size) => *size,
			Contents::Folder(files) => {
				files.iter().map(|f| f.size()).sum()
			},
		}
	}
}
