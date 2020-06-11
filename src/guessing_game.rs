use rand::random;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

pub fn run() {
    let secret: u8 = random();

    println!("Random number generated...");

    loop {
        print!("Guess a number: ");
        std::io::stdout().flush().unwrap();
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input!");
                continue;
            }
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("You guess correctly!");
                break;
            }
        }
    }
}
