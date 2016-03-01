use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::str::Chars;
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
    // s = s.strip();

    // this is a vector of string literals, not Strings
    let split: Vec<&str> = s.split_whitespace().collect();

    interpret(split);
}

fn interpret(this: Vec<&str>){
    println!("{:?}",this);
    // println!("{}","blah" == String::from("blah"));
    let mut count: u64 = 0;
    loop{
        let start_ind = this.iter().position(|&x| x.trim() == format!("{}.",count));
        // println!("{:?}",start_ind);
        let finish_ind = this.iter().position(|&x| x.trim() == format!(".{}",count));
        // println!("{:?}",finish_ind);


        match (start_ind,finish_ind){
            (None,Some(x)) => panic!("didn't start operation #{:?}",count),
            (Some(x),None) => panic!("never closed operation #{:?}",count),
            // this will happen when the program ends
            (None,None) => break,
            _ => {
                for ind in start_ind.unwrap()..finish_ind.unwrap() + 1 { // you have to unwrap these because they're Some(thing)
                    for ch in this[ind].chars(){
                        if ch == "." {
                            continue;
                        }
                    }
                }
            }
        }
        count += 1;

    }
}
