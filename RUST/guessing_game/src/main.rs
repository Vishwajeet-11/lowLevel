use std::cmp::Ordering;
use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("please input your guess");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("failed");

    let guess: u32 = guess.trim().parse().expect("please type a number");

    println!("the secret number is {secret_number}");

    println!("you guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too Small"),
        Ordering::Greater => print!("Too Large"),
        Ordering::Equal => println!("you Win!!"),
    }
}
