use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let num = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("inputtt");
        let mut guess = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("failed to get user input");

        println!("you guess {guess}");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_err) => continue,
        };


        match guess.cmp(&num) {
            Ordering::Equal => {
                println!("win");
                break;
            }
            Ordering::Greater => println!("too high"),
            Ordering::Less => println!("too low"),
        }
    }
}
