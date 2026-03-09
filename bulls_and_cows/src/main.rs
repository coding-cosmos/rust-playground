use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Welcome to Bulls and Cows");
    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut attempts = 0;

    let mut input_data = String::new();

    println!("Please input a number: ");
    io::stdin()
        .read_line(&mut input_data)
        .expect("Failed to read line");

    loop {
        println!("Please input a number: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Faild to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a valid number");
                continue;
            }
        };

        attempts += 1;

        if guess < 1 || guess > 100 {
            println!("Please input a number between 1 and 100");
            continue;
        }

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("too small!!");
                if attempts > 5 {
                    println!("tips: you have tried {} times", attempts);
                }
            }
            Ordering::Greater => {
                println!("too big!!");
                if attempts > 5 {
                    println!("tips: you have tried {} times", attempts);
                }
            }
            Ordering::Equal => {
                println!("Congratulations you are right!!");
                println!("tips: you have tried {} times", attempts);
                break;
            }
        }

        if attempts == 10 {
            println!("You have tried 10 times, game over");
            break;
        }

    }
}
