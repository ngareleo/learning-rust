use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    // recreating the guessing game
    let secret = rand::thread_rng().gen_range(1..100);
    loop {
        println!("Enter guess");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Error occurred");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        match guess.cmp(&secret) {
            Ordering::Equal => {
                println!("Found"); 
                break;
            },
            Ordering::Greater => println!("Too hot"),   
            Ordering::Less => println!("Too cold"),
        };    
    }
}
