//use std::io;
use std::env;

fn main(){

    println!("Guess The Number!");

    let a = env::args().nth(1)
                       .expect("Enter a Number");

    println!("Your Guess is: {}", a);






}