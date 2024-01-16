use std::process::exit;

use aoc2024::Settings;
use clap::Parser;

fn main() {
	let mut settings = Settings::parse();
	if let Err(e) = settings.run() {
		eprintln!("Runner: {e}");
		exit(1);
	}
}
