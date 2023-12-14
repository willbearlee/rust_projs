use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // Rust macro, ! means calling a marco instead of a normal function.
    // Marco is not the same as functions.
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is :{secret_number}");

    loop {
        println!("Please input your guess");

        // use let to create variable.
        // By default, variables are immutable.
        let apple = 5; // immutable
        // The :: syntax in the ::new line indicates that new is an associated function of the String type.

        // This new function creates a new, empty string. You’ll find a new function on many types because it’s a common name for a function that makes a new value of some kind.
        let mut guess = String::new(); // mutable

        //like variables, references are immutable by default. Hence, you need to write &mut guess rather than &guess to make it mutable
        io::stdin()
            .read_line(&mut guess)
            .expect("Fail to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
