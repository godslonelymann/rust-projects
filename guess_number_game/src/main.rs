use std::io;
use rand::Rng;

fn main(){
    println!("Guess the Number!");

    let random_num: i64 = rand::thread_rng().gen_range(1..=100);

    
    loop{
        println!("Random Number: {random_num}");
        println!("Enter your guess.");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Invalid type entered");

        let guess: i64 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter valid number");
                continue;
            }
        };

        println!("Guess: {guess}");

        if guess < random_num {
            println!("The guess is low.");
        } else if guess > random_num {
            println!("The guess is high");
        } else if guess == random_num {
            println!("You Win!, {guess} == {random_num}");
            break;
        }
    }


}