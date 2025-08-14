use std::env;

fn main(){

    let args: Vec<String> = env::args().collect();

    for arg in args.iter().skip(1) {
        println!("Args: {}", arg);
    }

    // println!("Total user args: {arg}");
}


//enumerate() - adds index to each command line arg - not necessary just gives index values
//iter() - look at the items without removing them from the vector