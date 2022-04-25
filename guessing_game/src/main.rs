use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number!");

    let mut upper_value: u32 = 101;
    let mut lower_value: u32 = 1;

    loop { // Creates an infinite loop
        let secret_number = rand::thread_rng().gen_range(lower_value..upper_value); // first inclusive, second non-inclusive. Can use  1..=100 for inclusive

        println!("The secret number is: {}", secret_number);
        println!("Please input your guess.");

        let mut guess = String::new(); // let declares variable, mut makes this variable mutable
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // Expect catches Err variants of Result enum and prints
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,  // If it's OK, get the number and return it
            Err(_) => continue, // _ is catchall value, no matter what err contains, this is what we want to do.   
        };
                        


        
        println!("You guessed: {}", guess); // ! because println is a macro (so {} can work?)

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small");
                match guess.cmp(&lower_value){
                    Ordering::Greater => lower_value = guess,
                    Ordering::Less => lower_value = lower_value,
                    Ordering::Equal => lower_value = lower_value // How to ignore other conditions in match?
                }
            },
            Ordering::Greater => {
                println!("Too big!");
                match guess.cmp(&upper_value){
                    Ordering::Less => upper_value = guess,
                    Ordering::Greater => upper_value = upper_value,
                    Ordering::Equal => upper_value = upper_value
                }
            },
            Ordering::Equal => {
                println!("You win!");
                break; // Break out of loop
            }
        }
    }
}

// This completes Ch 2 of The Rust Book. Pretty cool so far.
// Really like the way in which exceptions are handled. Think rustfmt is extremely useful, know I would use it all the time.
// Interesting how type declaration occurs after var name.
// Hope match isn't only form of if, pretty verbose when I could just say if a<b as opposed to match a b Ordering::Less =>
// Also appreciate putting in the effort and trying to learn keyboard shortcuts. Trying to optimize workflow.
// Good job!
// 4/25/22 2:56 AM
