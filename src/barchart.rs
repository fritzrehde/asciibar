use crate::config::Config;

pub struct BarChart {
	filled: Segment,
	half: Segment,
	empty: Segment,
	border: Option<char>,
	newline: bool,
}

struct Segment {
	length: u32,
	display: char,
}

impl BarChart {
	pub fn new(config: Config) -> Self {
		let (len1, len2) = config.percentage.split_into_ranges(config.length);
		Self {
			filled: Segment {
				length: len1,
				display: config.char_filled,
			},
			half: Segment {
				length: len2,
				display: config.char_half,
			},
			empty: Segment {
				length: len2,
				display: config.char_empty,
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
		for _ in 0..self.empty.length {
			chart.push(self.empty.display);
		}
		if let Some(border) = self.border {
			chart.insert(0, border);
			chart.push(border);
		}
		if self.newline {
			// TODO: might not work on all platforms (looking at you, windows)
			chart.push('\n');
		}

		// TODO: find faster way of printing to stdout
		print!("{}", chart);
	}
}
