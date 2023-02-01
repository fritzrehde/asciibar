use anyhow::{bail, Result};

// internally saved as value between 0.0 and 1.0
pub struct Percentage(f64);

impl Percentage {
	fn new(percentage: f64, min: f64, max: f64) -> Option<Self> {
		(min..=max)
			.contains(&percentage)
			.then(|| Self((percentage - min) / (max - min)))
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
				// TODO: improve with Rust syntax sugar
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

	pub fn split_value(&self, value: usize) -> (usize, usize, usize) {
		// TODO: look into max and min values of floats and ints
		// TODO: find std lib method that floors and casts to u32 at once, seems error-prone this way
		let exact_range = value as f64 * self.0;
		let range1 = exact_range.floor() as usize;
		let range2 = (exact_range.fract() >= 0.5).into();
		let range3 = value - range1 - range2;
		(range1, range2, range3)
	}
}
