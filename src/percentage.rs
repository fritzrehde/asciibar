use anyhow::{anyhow, bail, Result};

// internally saved as value between 0.0 and 1.0
pub struct Percentage(f64);

impl Percentage {
	fn new(percentage: f64, min: f64, max: f64) -> Option<Self> {
		(min..=max)
			.contains(&percentage)
			// TODO: no checks on overflows, might panic
			.then(|| Self((percentage - min) / (max - min)))
	}

	pub fn parse(percentage: f64, min: Option<f64>, max: Option<f64>) -> Result<Self> {
		Ok(match (min, max) {
			(Some(min), Some(max)) => {
				Self::new(percentage, min, max)
					.ok_or_else(|| anyhow!("The percentage must be in the range of the specified minimum and maximum values"))?
			},
			(None, None) => {
				Self::new(percentage, 0.0, 1.0)
					.or_else(|| Self::new(percentage, 0.0, 100.0))
					.ok_or_else(|| anyhow!("The percentage must be in the range of the specified minimum and maximum values"))?
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
