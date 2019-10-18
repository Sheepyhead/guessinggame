use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess a fuckin number bitch");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!(
        "The secret number is {} not very secret if you ask me",
        secret_number
    );

    loop {
        println!("Input the stupid number");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too smol"),
            Ordering::Greater => println!("Too tol!"),
            Ordering::Equal => {
                println!("Just rite fam");
                break;
            }
        }
    }
}
