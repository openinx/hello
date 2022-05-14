use std::io;

mod sum;
mod prime;
mod qsort;

fn main() {
    println!("Hello, world!");

    sum::for_loop();
    prime::is_prime(23);

    let mut guess: String = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to readline");
    
    println!("You guessed: {}", guess);
}
