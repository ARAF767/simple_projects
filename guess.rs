use rand::Rng;
use std::io::{self, Write};

use std::cmp::Ordering;

fn input() -> String {
    let mut x = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut x).expect("Something went wrong");
    return x;
}
fn main() {
    println!("Guess the number! ");

    let mut rng = rand::thread_rng();

    let secret_number = rng.gen_range(1..=100);

    loop {
        print!("Please input a number: ");

        let guess: u32 = match input().trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("too big"),
            Ordering::Less => println!("too small"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
