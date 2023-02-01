use snapbox::cmd::{cargo_bin, Command, OutputAssert};

// TODO: turn into macro
fn command(cmd: &str) -> OutputAssert {
	let mut args = cmd.split_whitespace();
	Command::new(cargo_bin(args.next().unwrap()))
		.args(args)
		.assert()
}

// TODO: group test cases somehow
#[test]
fn test_no_arguments() {
	command("asciibar").failure();
}

#[test]
fn test_only_min_or_max() {
	command("asciibar --min 10 -- 42").failure();
	command("asciibar --max 10 -- 42").failure();
}

#[test]
fn test_out_of_default_range_percentage() {
	command("asciibar -- -1").failure();
	command("asciibar -- 101").failure();
}

#[test]
fn test_out_of_min_max_range_percentage() {
	command("asciibar --min 10 --max 20 -- 21").failure();
	command("asciibar --min 10 --max 20 -- 9").failure();
}

#[test]
fn test_negative_percentage() {
	command("asciibar --min=-15 --max=-5 -10").failure();
	command("asciibar --min=-15 --max=-5 -- -10").success();
}

#[test]
fn test_negative_min_max() {
	command("asciibar --min -15 --max -5 -- -10").failure();
	command("asciibar --min=-15 --max=-5 -- -10").success();
}

// TODO: uses default block chars, find way to use generic "testing" stdout syntax to allow for changes of default values
#[test]
fn test_default() {
	command("asciibar -- 0.55").stdout_eq("█████▌    ");
}

#[test]
fn test_custom() {
	command("asciibar --filled = --half-filled > --empty - -- 0.55").stdout_eq("=====>----");
}

#[test]
fn test_custom_half_filled() {
	command("asciibar --half-filled > -- 0.55").stdout_eq("█████>    ");
}

#[test]
fn test_custom_without_half_filled() {
	command("asciibar --filled = --empty - -- 0.55").stdout_eq("=====-----");
}
