use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to guessing the number! \n");
    let secret_number = rand::thread_rng().gen_range(0, 101);

    loop {
        println!("number generated \n");
        println!("Guess the number: ");
        let mut guess = String::new();

        io::stdin() 
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed: {} \n", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small! \n"),
            Ordering::Greater => println!("Too big \n"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }


}
