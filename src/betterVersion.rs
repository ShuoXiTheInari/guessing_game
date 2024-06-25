use std::io;
use rand::random;

// 提取常量用于国际化和维护性
const GUESS_PROMPT: &str = "Please input your guess.";
const WIN_MESSAGE: &str = "You win!";
const TOO_SMALL_MESSAGE: &str = "Too small!";
const TOO_BIG_MESSAGE: &str = "Too big!";
const INVALID_INPUT_MESSAGE: &str = "Please input a number!";

fn main() {
    let secret_number = generate_secret_number();

    println!("Guess the number!");

    let mut guess = String::new(); 

    loop {
        println!("{}", GUESS_PROMPT);

        match io::stdin().read_line(&mut guess) {
            Ok(_num_bytes) => {
                println!("You guessed: {}", guess.trim());
                if let Ok(num) = parse_guess(&guess) {
                    match num.cmp(&secret_number) {
                        std::cmp::Ordering::Less => println!("{}", TOO_SMALL_MESSAGE),
                        std::cmp::Ordering::Greater => println!("{}", TOO_BIG_MESSAGE),
                        std::cmp::Ordering::Equal => {
                            println!("{}", WIN_MESSAGE);
                            break;
                        }
                    }
                } else {
                    println!("{}", INVALID_INPUT_MESSAGE);
                }
                guess.clear(); // 清空字符串，复用内存
            }
            Err(e) => {
                eprintln!("Failed to read line: {}", e);
                break;
            }
        }
    }
}

// 生成1到100的随机数，确保边界条件被正确处理
pub fn generate_secret_number() -> i32 {
    let random_number = random::<i32>();
    (random_number % 100) + 1
}

// 解析用户输入为i32，处理无效输入
pub fn parse_guess(guess: &str) -> Result<i32, ()> {
    match guess.trim().parse::<i32>() {
        Ok(num) => Ok(num),
        Err(_) => Err(()),
    }
}