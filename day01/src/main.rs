use std::env;
use std::fs;

fn main() {
	let contents = fs::read_to_string("input-01.txt")
        .expect("Should have been able to read the input file");

	println!("{contents}");
}