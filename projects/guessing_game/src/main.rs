use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secert_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secert numbe is: {secert_number}");

    loop {
        println!("Please input your guess(1~100).");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // _ 是一个通配符值，本例中用来匹配所有 Err 值
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secert_number) {
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
