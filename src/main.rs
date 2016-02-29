use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
#[macro_use]
extern crate lazy_static;

lazy_static!{
    static ref reserved_operators: Vec<&'static str> = vec!["pause","output", "input", "finput", "sto", "dupe" ,"+", "-", "/", "*", "%", "?", "goto", "end", "rand", ">", "<", "==", "!="];
    static ref memory: HashMap<&'static str,Vec<&'static str>> = HashMap::new();
}

fn main() {
    let mut file = File::open("test.step")
        .expect("couldn't open file");
    // println!("here now!");
    let mut s = String::new();
    file.read_to_string(&mut s)
        .expect("couldn't read the file");

    // this is a vector of string literals, not Strings
    let split: Vec<&str> = s.split_whitespace().collect();

    interpret(split);
}

fn interpret(this: Vec<&str>){
    let count: u64 = 0;
    loop{
        let list_iter = this.iter();
        let start_ind = list_iter.position(|&x| x.equals(format!("{}.",count)));
        let finish_ind = list_iter.position(|&x| x.equals(format!(".{}",count)));
        count += 1;
    }
}
