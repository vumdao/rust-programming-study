use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    let secret_num = rand::thread_rng().gen_range(1, 101);
    
    loop {
        println!("Input Your Guess nubmer.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to get guess input");
        
        println!("You guessed: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {println!("Wrong input, please input number"); continue;},
        };

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {println!("Bingo!"); break;}
        }
    }
}
