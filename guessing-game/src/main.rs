use std::io;
// from std import io, in familiar neanderthal

fn main() {

	println!("guess the number!");
	println!("please input your guess.");

	let mut guess = String::new();
		// let x = 1; // creates immutable var
		// let mut x = 1; // creates mutable var
		// Type::staticmethod(); :: signals an associated function of a type, rather than a .method() of its instance

	io::stdin() 
		// same thing here - stdin() method of the io module
		// without the global use import, could here also call std::io::stdin (ew) to get a std::io::Stdin instance back
		.read_line(&mut guess)
			// --> returns an io::Result (an enum, with variants Ok and Err)
			// & signifies a reference rather than moving/re-memming the obj itself
			// references are immutable by default - &mut guess makes a mutable reference
			// whereas &guess would make an immutable reference
		.expect("something went wrong - failed to read line");
		// i.e. calling io::Result.expect(self, msg: &str) instead of writing error handling
		// if Ok, returns the Ok value and consumes self
		// if Err, causes panic and displays the message passed to .expect()

	println!("your guess was: {}", guess);
	// fairly familiar string interpolation

}
