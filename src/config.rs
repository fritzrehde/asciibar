use crate::percentage::Percentage;
use anyhow::Result;
use clap::Parser;

pub struct Config {
	pub percentage: Percentage,
	pub char_filled: char,
	pub char_half: char,
	pub char_empty: char,
	pub length: u32,
	pub border: Option<char>,
	pub newline: bool,
}

impl Config {
	pub fn parse() -> Result<Self> {
		let cli = ClapConfig::parse();
		Ok(Self {
			percentage: Percentage::parse(cli.percentage, cli.min_percentage, cli.max_percentage)?,
			char_filled: cli.char_filled,
			char_half: cli.char_half,
			char_empty: cli.char_empty,
			length: cli.length,
			border: cli.border,
			newline: cli.newline,
		})
	}
}

#[derive(Parser)]
#[clap(version, about)]
struct ClapConfig {
	/// Percentage to be displayed
	percentage: f64,

	/// Length of bar chart measured in number of characters
	#[arg(short, long, value_name = "WIDTH", default_value_t = 10)]
	length: u32,

	/// ASCII character representing the filled out blocks of the bar
	#[arg(long = "filled", value_name = "CHAR", default_value_t = '█')]
	char_filled: char,

	/// ASCII character representing the half filled out blocks of the bar
	#[arg(long = "half", value_name = "CHAR", default_value_t = '▌')]
	char_half: char,

	/// ASCII character representing the empty blocks of the bar
	#[arg(long = "empty", value_name = "CHAR", default_value_t = ' ')]
	char_empty: char,

	/// Minimum value of custom percentage scale
	#[arg(long = "min", value_name = "MIN")]
	min_percentage: Option<f64>,

	/// Maximum value of custom percentage scale
	#[arg(long = "max", value_name = "MAX")]
	max_percentage: Option<f64>,

	/// Border character around bar chart
	#[arg(long = "border", value_name = "MAX")]
	border: Option<char>,

	/// Print newline after bar chart
	#[arg(long = "newline")]
	newline: bool,
}
