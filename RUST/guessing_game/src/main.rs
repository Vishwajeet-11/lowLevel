use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("please input your guess");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("failed");
    println!("the secret number is {secret_number}");

    println!("you guessed: {}", guess);
}
