#[allow(non_snake_case)]

use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::io;
// use std::str::Chars;



fn len(s: String) -> i64{
    let mut count: i64 = 0;
    for _ in s.chars() {
        count += 1;
    }
    count
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

    // let reserved_operators: Vec<&'static str> = vec!["pause","output", "input", "finput", "sto", "dupe" ,"+", "-", "/", "*", "%", "?", "goto", "end", "rand", ">", "<", "==", "!="];
    // we have to use Strings becuase otherwise the borrowing gets all hosed
    let mut memory: HashMap<String,Vec<String>> = HashMap::new();
    memory.insert(String::from("main"),Vec::new());

    let mut current_memory = String::from("main");

    // println!("{:?}",this);
    // println!("{}","blah" == String::from("blah"));
    let mut count: i64 = 0;
    'top: loop{
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
                    if !is_step {
                        // put all the other operator logic in here
                        let res = this[ind].find("?");
                        let mut current_operator = String::from(this[ind]);
                        match res{
                            Some(x) => {current_operator.remove(x);},
                            _ => {();}
                        }
                        // doing some shady shit with short circuiting here nbd
                        if((res != None && memory.get_mut(&current_memory).unwrap().pop().unwrap() == "yea") || res == None)  {
                            match &*current_operator {
                                "+" => {
                                    let mem = memory.get_mut(&current_memory).unwrap();
                                    let str_one: String = mem
                                        .pop()
                                        .unwrap();
                                    let one: i32 = str_one
                                        .trim()
                                        .parse()
                                        .expect("your argument to + wasn't a number!");
                                    let str_two: String = mem
                                        .pop()
                                        .unwrap();
                                    let two: i32 = str_two
                                        .trim()
                                        .parse()
                                        .expect("your argument to + wasn't a number!");
                                    let ret = one + two;
                                    let res = String::from(&*format!("{}",ret));
                                    mem.push(res);
                                },

                                "-" => {
                                    let mem = memory.get_mut(&current_memory).unwrap();
                                    let str_one: String = mem
                                        .pop()
                                        .unwrap();
                                    let one: i32 = str_one
                                        .trim()
                                        .parse()
                                        .expect("your argument to + wasn't a number!");
                                    let str_two: String = mem
                                        .pop()
                                        .unwrap();
                                    let two: i32 = str_two
                                        .trim()
                                        .parse()
                                        .expect("your argument to + wasn't a number!");
                                    let ret = one - two;
                                    let res = String::from(&*format!("{}",ret));
                                    mem.push(res);
                                },

                                "/" => {
                                    let mem = memory.get_mut(&current_memory).unwrap();
                                    let str_one: String = mem
                                        .pop()
                                        .unwrap();
                                    let one: i32 = str_one
                                        .trim()
                                        .parse()
                                        .expect("your argument to + wasn't a number!");
                                    let str_two: String = mem
                                        .pop()
                                        .unwrap();
                                    let two: i32 = str_two
                                        .trim()
                                        .parse()
                                        .expect("your argument to + wasn't a number!");
                                    let ret = one / two;
                                    let res = String::from(&*format!("{}",ret));
                                    mem.push(res);
                                },

                                "*" => {
                                    let mem = memory.get_mut(&current_memory).unwrap();
                                    let str_one: String = mem
                                        .pop()
                                        .unwrap();
                                    let one: i32 = str_one
                                        .trim()
                                        .parse()
                                        .expect("your argument to + wasn't a number!");
                                    let str_two: String = mem
                                        .pop()
                                        .unwrap();
                                    let two: i32 = str_two
                                        .trim()
                                        .parse()
                                        .expect("your argument to + wasn't a number!");
                                    let ret = one * two;
                                    let res = String::from(&*format!("{}",ret));
                                    mem.push(res);
                                },

                                "%" => {
                                    let mem = memory.get_mut(&current_memory).unwrap();
                                    let str_one: String = mem
                                        .pop()
                                        .unwrap();
                                    let one: i32 = str_one
                                        .trim()
                                        .parse()
                                        .expect("your argument to + wasn't a number!");
                                    let str_two: String = mem
                                        .pop()
                                        .unwrap();
                                    let two: i32 = str_two
                                        .trim()
                                        .parse()
                                        .expect("your argument to + wasn't a number!");
                                    let ret = one % two;
                                    let res = String::from(&*format!("{}",ret));
                                    mem.push(res);
                                },

                                "output" => {
                                    let mem = memory.get_mut(&current_memory).unwrap();
                                    let thing = mem.pop().unwrap();
                                    println!("{:?}",thing);
                                },

                                "goto" => {
                                    println!("at the goto op! {:?}",memory[&current_memory]);

                                    let mem = memory.get_mut(&current_memory).unwrap();
                                    let ind: i64 = mem.pop()
                                        .unwrap()
                                        .parse()
                                        .expect(&*format!("your argument to goto on operation {} was not an integer!",count));
                                    // just subtract one so that when one is added its a okay
                                    count = ind - 1;
                                },

                                "end" => {
                                    break 'top;
                                },

                                "dupe" => {
                                    let mem = memory.get_mut(&current_memory).unwrap();
                                    let val: &str = &*mem.pop().unwrap();
                                    mem.push(String::from(val));
                                    mem.push(String::from(val));
                                },

                                "<=" => {
                                    let first_mem_name = String::from(memory
                                        .get_mut(&current_memory)
                                        .unwrap()
                                        .pop()
                                        .unwrap());
                                    let second_mem_name = String::from(memory
                                        .get_mut(&current_memory)
                                        .unwrap()
                                        .pop()
                                        .unwrap());
                                    let val = memory
                                        .get_mut(&first_mem_name)
                                        .unwrap()
                                        .pop()
                                        .unwrap();
                                    memory
                                        .get_mut(&second_mem_name)
                                        .unwrap()
                                        .push(
                                            String::from(
                                                    val
                                                )
                                            );
                                },
                                // this is the same thing, just going the opposite direction
                                "=>" => {
                                    println!("swapping!");
                                    let second_mem_name = String::from(memory
                                        .get_mut(&current_memory)
                                        .unwrap()
                                        .pop()
                                        .unwrap());
                                    let first_mem_name = String::from(memory
                                        .get_mut(&current_memory)
                                        .unwrap()
                                        .pop()
                                        .unwrap());
                                    let val = memory
                                        .get_mut(&first_mem_name)
                                        .unwrap()
                                        .pop()
                                        .unwrap();
                                    memory
                                        .get_mut(&second_mem_name)
                                        .unwrap()
                                        .push(
                                            String::from(
                                                    val
                                                )
                                            );
                                },

                                "switch" => {
                                    println!("switching!");
                                    println!("contents of stack {}:{:?}",current_memory,memory.get_mut(&current_memory).unwrap());
                                    let new = String::from(&*memory.get_mut(&current_memory).unwrap().pop().unwrap());
                                    let mut possible = false;
                                    for a in memory.keys(){
                                        if a == &new{
                                            possible = true;
                                            break;
                                        }
                                    }

                                    if possible {
                                        current_memory = new;
                                    }
                                    else{
                                        panic!(format!("the alternate stack you specified, {}, doesn't exist! spawn it first.",new));
                                    }
                                },

                                "spawn" => {
                                    let name = String::from(&*memory.get_mut(&current_memory).unwrap().pop().unwrap());
                                    memory.insert(name,Vec::new());
                                },

                                "input" => {
                                    let mut line = String::new();
                                    match io::stdin().read_line(&mut line) {
                                        Ok(_) => {();},
                                        Err(_) => {panic!("error reading from stdin!");}
                                    }
                                    memory.get_mut(&current_memory).unwrap().push(line);

                                },

                                "<" => {
                                    let mem_borrow = memory.get_mut(&current_memory).unwrap();
                                    // if there is an option that's a string, we're going to make it a number dammit
                                    let val1 = String::from(mem_borrow.pop().unwrap());
                                    let val2 = String::from(mem_borrow.pop().unwrap());
                                    match (val1.parse::<i64>(), val2.parse::<i64>()) {
                                        (Ok(x),Ok(y)) => {
                                            if y < x {
                                                mem_borrow.push(String::from("yea"));
                                            }
                                            else{
                                                mem_borrow.push(String::from("nope"));
                                            }
                                        },
                                        // they're trying to judge a number vs a string, then use the length of the string
                                        (Err(_),Ok(y)) => {
                                            if y < len(val1){
                                                mem_borrow.push(String::from("yea"));
                                            }
                                            else{
                                                mem_borrow.push(String::from("nope"));
                                            }
                                        },

                                        (Ok(x),Err(_)) => {
                                            if x < len(val2){
                                                mem_borrow.push(String::from("yea"));
                                            }
                                            else{
                                                mem_borrow.push(String::from("nope"));
                                            }
                                        },

                                        (Err(_),Err(_)) => {
                                            if len(val2) < len(val1){
                                                mem_borrow.push(String::from("yea"));
                                            }
                                            else{
                                                mem_borrow.push(String::from("nope"));
                                            }
                                        }
                                    }
                                    println!("{:?}",memory.get(&current_memory));
                                },

                                ">" => {
                                    // if there is an option that's a string, we're going to make it a number dammit
                                    let val2 = String::from(memory.get_mut(&current_memory).unwrap().pop().unwrap());
                                    let val1 = String::from(memory.get_mut(&current_memory).unwrap().pop().unwrap());
                                    match (val1.parse::<i64>(), val2.parse::<i64>()) {
                                        (Ok(x),Ok(y)) => {
                                            if y < x {
                                                memory.get_mut(&current_memory).unwrap().push(String::from("yea"));
                                            }
                                            else{
                                                memory.get_mut(&current_memory).unwrap().push(String::from("nope"));
                                            }
                                        },
                                        // they're trying to judge a number vs a string, then use the length of the string
                                        (Err(_),Ok(y)) => {
                                            if y < len(val1){
                                                memory.get_mut(&current_memory).unwrap().push(String::from("yea"));
                                            }
                                            else{
                                                memory.get_mut(&current_memory).unwrap().push(String::from("nope"));
                                            }
                                        },

                                        (Ok(x),Err(_)) => {
                                            if x < len(val2){
                                                memory.get_mut(&current_memory).unwrap().push(String::from("yea"));
                                            }
                                            else{
                                                memory.get_mut(&current_memory).unwrap().push(String::from("nope"));
                                            }
                                        },

                                        (Err(_),Err(_)) => {
                                            if len(val2) < len(val1){
                                                memory.get_mut(&current_memory).unwrap().push(String::from("yea"));
                                            }
                                            else{
                                                memory.get_mut(&current_memory).unwrap().push(String::from("nope"));
                                            }
                                        }
                                    }
                                    println!("{:?}",memory.get(&current_memory));
                                },

                                // "=" =>{
                                //     let val1 = String::from(memory.get)
                                // },

                                _ => {
                                    println!("pushed {:?} onto the stack {}",current_operator,current_memory);
                                    memory.get_mut(&current_memory).unwrap().push(String::from(current_operator));
                                }
                            }
                        }
                    }
                }
            }
        }
        count += 1;
        // println!("{:?}",memory["main"]);
    }
}
