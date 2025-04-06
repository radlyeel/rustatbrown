use std::io;
fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        // See Ch. 9 for better error handling
        // Here 'msg:' is added by VSCode
        .expect("Failed to read line");
    println!("You guessed: {}", guess);
}
