use rand::Rng;
use rand::thread_rng;
use std::cmp::Ordering;
use std::io::stdin;

fn main() {
    let secret_number = thread_rng().gen_range(0..=100);

    loop {
        println!("Enter your guess: ");
        let mut guess = String::new();

        stdin().read_line(&mut guess).expect("Failed to read!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
            Ordering::Less => println!("Too Less!"),
            Ordering::Greater => println!("Too High!"),
        }
    }
}
