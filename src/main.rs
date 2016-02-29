use std::io::prelude::*;
use std::fs::File;

// static reserved_operators: Vec<&str> = vec!["pause","output", "input", "finput", "sto", "dupe" ,"+", "-", "/", "*", "%", "?", "goto", "end", "rand", ">", "<", "==", "!="];

fn main() {
    let mut file = File::open("test.step")
        .expect("couldn't open file");
    println!("here now!");
    let mut s = String::new();
    file.read_to_string(&mut s)
        .expect("couldn't read the file");

    // this is a vector of string literals, not Strings
    let split = s.split_whitespace().collect();


}

fn interpret(this: Vec<String>){

}
