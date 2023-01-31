use crate::config::Config;

pub struct BarChart {
	filled: Block,
	half_filled: Block,
	empty: Block,
	border: Option<char>,
	newline: bool,
}

struct Block {
	length: u32,
	display: char,
}

impl BarChart {
	pub fn new(config: Config) -> Self {
		let (block1, block2, block3) = config.percentage.split_value(config.length);
		Self {
			filled: Block {
				length: block1,
				display: config.filled,
			},
			half_filled: Block {
				length: block2,
				display: config.half_filled,
			},
			empty: Block {
				length: block3,
				display: config.empty,
			},
			border: config.border,
			newline: config.newline,
		}
	}

	pub fn draw(&self) {
		let mut chart = String::new();
		// TODO: heavily optimize, should be O(1) instead of O(n)
		for _ in 0..self.filled.length {
			chart.push(self.filled.display);
		}
		for _ in 0..self.half_filled.length {
			chart.push(self.half_filled.display);
		}
		for _ in 0..self.empty.length {
			chart.push(self.empty.display);
		}
		if let Some(border) = self.border {
			chart.insert(0, border);
			chart.push(border);
		}
		if self.newline {
			// TODO: might not work on all platforms (windows)
			chart.push('\n');
		}

		// TODO: find faster way of printing to stdout
		print!("{}", chart);
	}
}
