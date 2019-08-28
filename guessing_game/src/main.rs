use std::io::stdin;
use rand::Rng; // brings gen_range
use rand::thread_rng;
use std::cmp::Ordering;

fn main() {
    println!("========= Guess the number =========");
    let secret_number = thread_rng().gen_range(1, 101);
    println!("The secret number is {}", secret_number);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        let _return_from_read_line = stdin().read_line(&mut guess)
            .expect("Failed to read line"); // result is the length of the input.

        let guess: u32 = match guess.trim().parse() {// shadowing
            Ok(num) => num,
            Err(reason) => {
                println!("Input not accepted: {}", reason);
                continue;
            }
        };

        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("💡 Too small!"),
            Ordering::Greater => println!("💡 Too big!"),
            Ordering::Equal => {
                println!("🎉 You win!");
                break;
            },
        }
        // println!("P.S. _return_from_read_line = {}", return_from_read_line);
    };

}