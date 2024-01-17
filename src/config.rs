use crate::percentage::Percentage;
use anyhow::Result;
use clap::Parser;

const FILLED: char = '█';
const HALF_FILLED: char = '▌';
const EMPTY: char = ' ';

pub struct Config {
    pub percentage: Percentage,
    pub filled: char,
    pub half_filled: char,
    pub empty: char,
    pub length: usize,
    pub border: Option<char>,
    pub newline: bool,
}

impl Config {
    pub fn parse() -> Result<Self> {
        let cli = Cli::parse();

        // Only use default half filled char if no chars provided => looks out of place otherwise
        let (filled, half_filled, empty) = match (cli.filled, cli.half_filled, cli.empty) {
            (None, None, None) => (FILLED, HALF_FILLED, EMPTY),
            (filled, half, empty) => {
                let empty = empty.unwrap_or(EMPTY);
                (filled.unwrap_or(FILLED), half.unwrap_or(empty), empty)
            }
        };

        Ok(Self {
            percentage: Percentage::parse(cli.percentage, cli.min_percentage, cli.max_percentage)?,
            filled,
            half_filled,
            empty,
            length: cli.length,
            border: cli.border,
            newline: cli.newline,
        })
    }
}

#[derive(Parser)]
#[clap(version, about, author)]
struct Cli {
    /// Percentage to be displayed
    percentage: f64,

    /// Length of bar chart measured in number of characters
    #[arg(short, long, value_name = "UNSIGNED", default_value_t = 10)]
    length: usize,

    /// ASCII character representing the filled out blocks of the bar
    #[arg(long = "filled", value_name = "CHAR")]
    filled: Option<char>,

    /// ASCII character representing the half_filled filled out blocks of the bar
    #[arg(long = "half-filled", value_name = "CHAR")]
    half_filled: Option<char>,

    /// ASCII character representing the empty blocks of the bar
    #[arg(long = "empty", value_name = "CHAR")]
    empty: Option<char>,

    /// Minimum value of custom percentage scale
    #[arg(long = "min", value_name = "FLOAT")]
    min_percentage: Option<f64>,

    /// Maximum value of custom percentage scale
    #[arg(long = "max", value_name = "FLOAT")]
    max_percentage: Option<f64>,

    /// Border character around bar chart
    #[arg(short, long = "border", value_name = "CHAR")]
    border: Option<char>,

    /// Print newline after bar chart
    #[arg(short, long = "newline")]
    newline: bool,
}
