use anyhow::{bail, Result};

// internally saved as value between 0.0 and 1.0
pub struct Percentage(f64);

impl Percentage {
	fn new(percentage: f64, min: f64, max: f64) -> Option<Self> {
		// TODO: improve with Rust syntax sugar
		if (min..=max).contains(&percentage) {
			Some(Self((percentage - min) / (max - min)))
		} else {
			None
		}
	}

	pub fn parse(percentage: f64, min: Option<f64>, max: Option<f64>) -> Result<Self> {
		Ok(match (min, max) {
			(Some(min), Some(max)) => {
				// TODO: improve with Rust syntax sugar
				match Self::new(percentage, min, max) {
					Some(perc) => perc,
					None => bail!("The percentage must be in the range of the specified minimum and maximum values"),
				}
			},
			(None, None) => {
				if let Some(perc) = Self::new(percentage, 0.0, 1.0) {
					perc
				} else if let Some(perc) = Self::new(percentage, 0.0, 100.0) {
					perc
				} else {
					bail!("The percentage must be either in the range 0.0 to 1.0 or 0.0 to 100.0");
				}
			}
			_ => bail!("When using a custom percentage scale, both the minimum and maximum values must be provided"),
		})
	}

	pub fn split_into_ranges(&self, value: u32) -> (u32, u32) {
		// TODO: look into max and min values of floats and ints
		// TODO: find std lib method that floors and casts to u32 at once, seems error-prone this way

		let split = (value as f64 * self.0).floor() as u32;

		// TODO: normal "-" safe?
		(split, value - split)
	}
}
