use std::io;
	// from std import io, in familiar neanderthal
use std::cmp::Ordering;
	// Ordering is also an enum
use rand::Rng;
	// Rng is a trait and defines methods that need to be in scope for us to use them
	// paired with their usage below, i don't fully understand this yet. 
	// is it similar to implicit importing?

fn main() {

	println!("Try to guess the secret number! (1-100)");
	let secret_num = rand::thread_rng().gen_range(1, 101);
		// thread_rng() is local to current exec thread, and seeded by OS
		// gen_range is a method defined by Rng trait 
		// note that gen_range is inclusive on lower bound and exclusive on upper bound

	// println!("DEBUG -- the secret number is: {}", secret_num);

	loop {

		println!("Guess a number.");

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
			.expect("Something went wrong - failed to read line");
				// i.e. calling io::Result.expect(self, msg: &str) instead of writing error handling
				// if Ok, returns the Ok value and consumes self
				// if Err, causes panic (crashes) and displays the message passed to .expect()

		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_)  => continue,
		};
			// shadows the prior guess var, basically reassigning to a var of the same name
			// trims whitespace on the String guess (e.g. the \n from pressing enter on input),
			// then parses a string into the provided type (here, a u32 number)
			// bc parse is so general it can cause problems with type inference,
			// so sometimes instead of let x: u32 = "5".parse();, one will see "let x = parse::<u32>();"
			// finally, we handle Result[OK, Err] by ignoring invalid guesses and asking for another.

		let confirm = format!("You guessed {}.", guess);
		// don't know enough about allocation & ownership to declare this string in further detail
		// we'll do it this way for now: I don't own / can't change the string, nor do I want to

		match guess.cmp(&secret_num) {

			// match expression's arms have a) a pattern, b) code to run if that pattern is matched
			// cmp method compares two values and can be called on anything that can be compared
			// Ordering enum has Less, Greater, Equal, and method implementations
			Ordering::Less    => println!("{} Too small!", confirm),
			Ordering::Greater => println!("{} Too big!",   confirm),
			Ordering::Equal   => {
				println!("You got it! The secret number was {}.", secret_num);
				break;

			}

		}

	}

}
