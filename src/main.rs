use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("!Welcome to the guessing game!\n");

    // generate secret number for gussing
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    loop {
        // take user input
        println!("\nPlease enter your guess:");
        let mut guessed_number = String::new();
        io::stdin()
            .read_line(&mut guessed_number)
            .expect("Unable to read the input!");

        // parse input to number
        let guessed_number: u32 = match guessed_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // compare secret and gussed numbers
        match guessed_number.cmp(&secret_number) {
            Ordering::Less => {
                if secret_number - guessed_number > 10 {
                    println!("=> Too Small!")
                } else {
                    println!("=> Small! Guess little large number.\n");
                }
            }
            Ordering::Greater => {
                if guessed_number - secret_number > 10 {
                    println!("=> Too Big!")
                } else {
                    print!("=> Big! Guess little smaller number.\n")
                }
            }
            Ordering::Equal => {
                println!("Congratulation! You win!");
                break;
            }
        }
    }
}
