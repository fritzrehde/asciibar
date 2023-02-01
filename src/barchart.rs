use crate::config::Config;

pub struct BarChart {
	filled: Block,
	half_filled: Block,
	empty: Block,
	border: Block,
	newline: Block,
}

#[derive(Clone)]
struct Block {
	length: usize,
	display: char,
}

impl Block {
	fn new(length: usize, display: char) -> Self {
		Self { length, display }
	}

	fn draw(&self) -> Vec<char> {
		vec![self.display; self.length]
	}
}

impl BarChart {
	pub fn new(config: Config) -> Self {
		let (block1, block2, block3) = config.percentage.split_value(config.length);
		Self {
			filled: Block::new(block1, config.filled),
			half_filled: Block::new(block2, config.half_filled),
			empty: Block::new(block3, config.empty),
			border: Block::new(config.border.is_some().into(), config.border.unwrap_or(' ')),
			newline: Block::new(config.newline.into(), '\n'),
		}
	}

	pub fn draw(self) -> String {
		// TODO: try to avoid clone of border/don't use border twice
		[
			self.border.clone(),
			self.filled,
			self.half_filled,
			self.empty,
			self.border,
			self.newline,
		]
		.iter()
		.flat_map(Block::draw)
		.collect()
	}
}
