use std::{env::args, fs::read_to_string};

use rle_rs::rle;

fn main() {
	let s = read_to_string(args().nth(1).unwrap()).unwrap();

	for a in rle(&s) {
		println!("{a:?}");
	}
}
