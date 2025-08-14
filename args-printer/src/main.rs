use std::env;

fn main(){

    let args: Vec<String> = env::args().collect();

    for (i, arg) in args.iter().skip(1).enumerate() {
        println!("args[{}]: {arg}", i + 1);
    }

    println!("Total user args: {}", args.len() - 1);
}