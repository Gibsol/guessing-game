use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess a number!");
    
    const LIMIT: u32 = 3;
    let mut counter = 0;
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please, enter your guess");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Error while reading the string");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You have guessed the: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("The number is too low"),
            Ordering::Greater => println!("The number is too big"),
            Ordering::Equal => {    
                println!("You win");
                break;
            }
        }

        counter += 1;

        if counter == LIMIT {
            println!("You lost!");
            break;
        }
    }
}