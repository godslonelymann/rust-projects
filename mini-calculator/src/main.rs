use std::env;


fn main() {
    
    let operation  = env::args().nth(1).unwrap();


    //.expect("msg here") can be used as substitute to .unwrap()
    //.expect() carries the error msg that is shown to user if any error occurs.
    //.unwrap() is just like if if is success then show the output; if unsuccessful then just crash the program.
    //.expect() is good to use as it shows the error msg and does'nt crash the program.
    
    let a: f32 = env::args().nth(2).expect("Enter an integer").parse().expect("Not a valid number");
    //let a: f32 = env::args().nth(2).unwrap().parse().unwrap();

    let b: f32 = env::args().nth(3).expect("Enter an integer").parse().expect("Not a valid number");
    //let b: f32 = env::args().nth(3).unwrap().parse().unwrap();


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