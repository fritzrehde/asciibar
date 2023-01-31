use snapbox::cmd::{cargo_bin, Command};

#[test]
fn test_no_options() {
	Command::new(cargo_bin("asciibar")).assert().failure();
}

#[test]
fn test_simple() {
	Command::new(cargo_bin("asciibar"))
		.args(&["0.5"])
		.assert()
		.stdout_eq("█████     ");
}
