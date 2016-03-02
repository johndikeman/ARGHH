#[allow(non_snake_case)]

use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
// use std::str::Chars;


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

    interpret(&split);
}

fn interpret<'a>(this: &'a Vec<&str>){

    let reserved_operators: Vec<&'static str> = vec!["pause","output", "input", "finput", "sto", "dupe" ,"+", "-", "/", "*", "%", "?", "goto", "end", "rand", ">", "<", "==", "!="];

    let mut memory: HashMap<&'static str,Vec<&str>> = HashMap::new();
    memory.insert("main",Vec::new());

    // println!("{:?}",this);
    // println!("{}","blah" == String::from("blah"));
    let mut count: u64 = 0;
    loop{
        let start_ind = this.iter().position(|&x| x.trim() == format!("{}.",count));
        // println!("{:?}",start_ind);
        let finish_ind = this.iter().position(|&x| x.trim() == format!(".{}",count));
        // println!("{:?}",finish_ind);


        match (start_ind,finish_ind){
            (None,Some(_)) => panic!("didn't start operation #{:?}",count),
            (Some(_),None) => panic!("never closed operation #{:?}",count),
            // this will happen when the program ends
            (None,None) => break,
            (Some(_),Some(_)) => {
                for ind in start_ind.unwrap()..finish_ind.unwrap() + 1 { // you have to unwrap these because they're Some(thing)
                    let mut is_step: bool = false;
                    for ch in this[ind].chars(){
                        // iterate through all the characters in the word and see if theyre a period
                        if ch == '.' {
                            // println!("got a number!");
                            is_step = true;
                            break;
                        }
                    }
                    if !is_step{
                        //put all the other operator logic in here
                        let peice: &'a str = this[ind];
                        memory.get_mut("main").unwrap().push(&*peice);
                    }
                }
            }
        }
        count += 1;
        println!("{:?}",memory["main"]);
    }
}
