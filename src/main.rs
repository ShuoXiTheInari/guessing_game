use std::io;
use rand::random;

fn main() {

    let secret_number = random::<i32>() % 100 + 1;

    println!("Guess the number!");
    let mut guess = String::new();

    loop{
    println!("Please input your guess.");
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
    let num = match guess.trim().parse::<i32>() {
        Ok(num) => num,
        Err(_) => {
            println!("Please input a number!");
            continue;
        }
    };
    match num.cmp(&secret_number) {
        std::cmp::Ordering::Less => {
            println!("Too small!");
        
        },
        std::cmp::Ordering::Greater => {
            println!("Too big!");
        },
        std::cmp::Ordering::Equal => {
            println!("You win!");
            break;
        },
    };
    guess.clear();
    }   
}

