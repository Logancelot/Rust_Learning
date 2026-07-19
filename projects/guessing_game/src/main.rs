use std::io;

fn main() {
    println!("Guess the Number!");
    println!("Plz input your guess noob");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read ine");
    println!("you guessed: {guess}");
}
1