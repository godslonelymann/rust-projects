use std::env;


fn main() {
    
    let operation  = env::args().nth(1).expect("Missing operation as input");


    let a: f32 = env::args().nth(2).expect("No number given as input").parse().expect("not a valid number");
    let b: f32 = env::args().nth(3).expect("No number given as input").parse().expect("not a valid number");

    let result = match operation.as_str() {
        "add" => a + b,
        "sub" => a - b,
        "mul" => a * b,
        "div" => {
            if b == 0.0{
                println!("Division by zero error");
                return;
            }

            a / b
        }
        _ => {
            println!("Invalid Operation!");
            return;

        }
    };

    println!("Result: {}", result);
}