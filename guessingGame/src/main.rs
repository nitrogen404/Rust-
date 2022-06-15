use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    loop {
        println!("Guess any number between 1 to 100");
        println!("Enter your guessed number ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("invalid error");
        let guess: u32 = guess.trim().parse().expect("Invalid input");

        if guess > 100 || guess < 1 {
            println!("number must be less than or equal to 100 and greater than 0");
            break;
        }
        else { 
            println!("You guessed {}", guess);
        }

        let secret_number = rand::thread_rng().gen_range(1..101);
        println!("Random number is {}", secret_number);
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"), 
            Ordering::Greater => println!("Too big"), 
            Ordering::Equal => {println!("you guessed it correct!!"); break;},
        }
    }
}