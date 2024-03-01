use std::io;
// After adding a line in [dependencies] of cargo.toml, we can do this:
use rand::Rng;
// For comparisons
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // get a  random int frm 1 to 100, inclusive (note = range spec)
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // For debugging
    // println!("The secret number is: {secret_number}");

    // Allow multiple guesses
    loop {

        println!("Please input your guess.");

        // Declare a variable for user input
        let mut guess = String::new();

        io::stdin()
            // Read a line of input
            .read_line(&mut guess)
            // Check for error in a Result (an enum with variants Ok and Err)
            .expect("Failed to read line");

        // Replace String guess with its u32 equivalent; ignore invalids
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Report to user
        // {} as placeholder; formatting codes to come later
        println!("You guessed: {guess}");

        // test response
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            //Quit on correct guess
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}
