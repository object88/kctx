use kctxd::args::{self, Args};

fn main() {
	let a: Args = args::new().unwrap();
	a.parse();
	// println!("Hello, world!");
}
