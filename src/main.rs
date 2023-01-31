mod barchart;
mod config;
mod percentage;

use anyhow::Result;
use barchart::BarChart;
use config::Config;

fn main() -> Result<()> {
	BarChart::new(Config::parse()?).draw();
	Ok(())
}
