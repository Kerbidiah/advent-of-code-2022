use rayon::prelude::*;

#[derive(Debug)]
pub struct VMap {
	pub vis_map: Vec<Vec<bool>>,
	pub y_max: usize,
	pub x_max: usize,
}

impl VMap {
	pub fn new(tree_map: &Vec<Vec<u8>>) -> Self {
		// initialize visibility map
		let mut vis_map = vec![vec![false; tree_map[0].len()]; tree_map.len()];

		let y_max = vis_map.len() - 1;
		let x_max = vis_map[0].len() - 1;

		// mark borders as visible
		vis_map[0] = vec![true; x_max + 1];
		vis_map[y_max] = vec![true; x_max + 1];
		for line in &mut vis_map {
			line[0] = true;
			line[x_max] = true;
		}

		Self {
			vis_map,
			y_max,
			x_max
		}
	}

	#[inline]
	fn top_visibility(&mut self, tree_map: &Vec<Vec<u8>>) {
		let mut max_top = tree_map[0].to_owned();

		for j in 1..self.y_max {
			for i in 1..self.x_max { // check and update max_top
				if tree_map[j][i] > max_top[i] {
					self.vis_map[j][i] = true;
					max_top[i] = tree_map[j][i];
				}
			}
		}
	}

	#[inline]
	fn bottom_visibility(&mut self, tree_map: &Vec<Vec<u8>>) {
		let mut max_bottom = tree_map[self.y_max].to_owned();

		for j in (1..self.y_max).rev() {
			for i in 1..self.x_max { // check and update max_top
				if tree_map[j][i] > max_bottom[i] {
					self.vis_map[j][i] = true;
					max_bottom[i] = tree_map[j][i];
				}
			}
		}
	}

	#[inline]
	fn left_visibility(&mut self, tree_map: &Vec<Vec<u8>>) {
		for j in 1..self.y_max {
			let mut left_max = tree_map[j][0];
			for i in 1..self.x_max { // check and update left_max
				if tree_map[j][i] > left_max {
					self.vis_map[j][i] = true;
					left_max = tree_map[j][i];
				}
			}
		}
	}

	#[inline]
	fn right_visibility(&mut self, tree_map: &Vec<Vec<u8>>) {
		for j in 1..self.y_max {
			let mut right_max = tree_map[j][self.x_max];
			for i in (1..self.x_max).rev() { // check and update right_max
				if tree_map[j][i] > right_max {
					self.vis_map[j][i] = true;
					right_max = tree_map[j][i];
				}
			}
		}
	}

	pub fn check_visible(&mut self, tree_map: &Vec<Vec<u8>>) {
		self.top_visibility(tree_map);
		self.bottom_visibility(tree_map);
		self.left_visibility(tree_map);
		self.right_visibility(tree_map);
	}

	pub fn count_visible(&self) -> usize {
		self.vis_map.par_iter()
			.flatten()
			.filter(|visible| **visible)
			.count()
	}
}