use std::io;
use rand::prelude::*;
use std::cmp::Ordering;

fn main() {
    let secret_number: u32 = rand::thread_rng().gen_range(1..100);
    println!("{}", secret_number);
    loop {
        let mut guess = String::new();
        println!("type a number: ");
        io::stdin()
            .read_line(&mut guess)
            .expect("input error!!");
        let guess_number: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("input not a number");
                continue;
            },
        };
        println!("you guess {}", guess_number);
        match guess_number.cmp(&secret_number) {
            Ordering::Less => println!("too lower!"),
            Ordering::Greater => println!("too greater!"),
            Ordering::Equal => {
                println!("win!");
                break;
            },
        };
    }


}