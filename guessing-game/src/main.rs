use std::io;
use rand;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let secret_numer = rand::thread_rng().gen_range(1..100);
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("TODO: panic message");

        let guess: u32 = match guess.trim().parse() {
            Ok( num ) => { num },
            Err(_) => continue,
        };

        println!("Guess {}", guess);

        match guess.cmp(&secret_numer) {
            Ordering::Less => println!("Less"),
            Ordering::Equal => {
                println!("win");
                break;
            }
            Ordering::Greater => println!("Grater")
        }
    }
}

