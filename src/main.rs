/*
    $ make run

    Guess the number!
    The secret number is: 77
    Please input your guess.
    10
    You guessed: 10
    Too small
    Please input your guess.
    leoo.j
    Invalid input value is given. 😡
    Please input your guess.
    77
    You guessed: 77
    You win 🌱
*/

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // generate secret number
    let secret_num = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_num}");

    loop {
        // input number
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        // "shadow" guess varialbe
        // convert guess type to u32
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input type is given. 😡");
                continue;
            }
        };
        println!("You guessed: {guess}");

        // compare secret num and your number
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win 🌱");
                break;
            }
        }
    }
}
