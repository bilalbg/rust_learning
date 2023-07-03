use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Generating a random number...");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut num_of_guess = 0;

    loop {
        println!("Guess a number: ");

        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // let answer = if secret_number == guess.trim().parse().unwrap() 
        //                 {"Congratulations!"} else 
        //                 {"Too bad!"};

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        num_of_guess += 1;

        // println!("The secret number was ..... {secret_number}! {answer}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("Correct!");
                break;
            },
            Ordering::Greater => println!("Too big!"),
        }
    }
    println!("You took {num_of_guess} guesses.");
}
