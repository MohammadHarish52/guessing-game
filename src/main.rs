
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..100);
    // Remove the line below to keep the secret number hidden
    println!("the lottery number  {}", secret_number);

    println!("Hello, world!");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please enter a number!");

    //compare
    //cmp takes the regerence to the compare variable
    match guess.cmp(&secret_number) {
        Ordering::Equal => println!("You win!"),
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big")
    }

    println!("You guessed: {}", guess);
}
