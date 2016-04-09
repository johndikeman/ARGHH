#![allow(non_snake_case)] // screw your snake case

use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::io;
// use std::str::Chars;


fn main() {
    let mut file = File::open("test.step")
        .expect("couldn't open file");
    // println!("here now!");
    let mut s = String::new();
    file.read_to_string(&mut s)
        .expect("couldn't read the file");
    // s = s.strip();

    // self.functions[function_name] is a vector of string literals, not Strings
    let mut interpreter = Ob::new(s);
    interpreter.populate();
    interpreter.interpret(String::from("main"));
}

fn len(s: String) -> i64{
    let mut count: i64 = 0;
    for _ in s.chars() {
        count += 1;
    }
    count
}

struct Ob{
    memory: HashMap<String,Vec<String>>,
    contents: String,
    functions: HashMap<String,Vec<String>>
}

impl Ob {
    pub fn new(c:String) -> Ob {
        let mem: HashMap<String,Vec<String>> = HashMap::new();
        Ob{ memory: HashMap::new(), contents: c, functions: mem}
    }

    pub fn set(&mut self, memory: HashMap<String,Vec<String>>,functions: HashMap<String,Vec<String>>){
        self.memory = memory;
        self.functions = functions;
    }

    pub fn ret(&mut self) -> (HashMap<String,Vec<String>>,HashMap<String,Vec<String>>) {
        (self.memory, self.functions)
    }

    pub fn populate(&mut self) {
        println!("here");
        println!("{}",self.contents);
        println!("{:?}",self.contents.split("#").collect::<Vec<&str>>());
        // let f: &mut HashMap<String,Vec<String>> = &mut HashMap::new();
        for function in self.contents.split("#") {
            println!("{}",function);
            if function != "" && function != " "{
                let n = function.split_whitespace().collect::<Vec<&str>>();
                let name: &str = n[0];
                self.functions.insert(String::from(name),
                    function.split_whitespace()
                    .map(|s| String::from(s)).collect()); // convert from Vec<&str> to Vec<String>
            }
        }

        self.memory.insert(String::from("main"), Vec::new());
    }

    pub fn interpret(&mut self, function_name: String){
        // let reserved_operators: Vec<&'static str> = vec!["pause","output", "input", "finput", "sto", "dupe" ,"+", "-", "/", "*", "%", "?", "goto", "end", "rand", ">", "<", "==", "!="];
        // we have to use Strings becuase otherwise the borrowing gets all hosed
        // self.memory.insert(String::from("main"),Vec::new());
        let mut current_memory = String::from("main");

        // println!("{:?}",self.functions[function_name]);
        // println!("{}","blah" == String::from("blah"));
        let mut count: i64 = 0;
        'top: loop{
            let start_ind = self.functions[&function_name].iter().position(|ref x| x.trim() == format!("{}.",count));
            // println!("{:?}",start_ind);
            let finish_ind = self.functions[&function_name].iter().position(|ref x| x.trim() == format!(".{}",count));
            // println!("{:?}",finish_ind);


            match (start_ind,finish_ind){
                (None,Some(_)) => panic!("didn't start operation #{:?}",count),
                (Some(_),None) => panic!("never closed operation #{:?}",count),
                // self.functions[function_name] will happen when the program ends
                (None,None) => break,
                (Some(_),Some(_)) => {
                    for ind in start_ind.unwrap()..finish_ind.unwrap() + 1 { // you have to unwrap these because they're Some(thing)
                        let mut is_step: bool = false;
                        for ch in self.functions[&function_name][ind].chars(){
                            // iterate through all the characters in the word and see if theyre a period
                            if ch == '.' {
                                // println!("got a number!");
                                is_step = true;
                                break;
                            }
                        }
                        if !is_step {
                            // put all the other operator logic in here
                            let res = self.functions[&function_name][ind].find("?");
                            let mut current_operator = String::from(self.functions[&function_name][ind].clone());
                            match res{
                                Some(x) => {current_operator.remove(x);},
                                _ => {();}
                            }
                            // doing some shady shit with short circuiting here nbd
                            if (res != None && self.memory.get_mut(&current_memory).unwrap().pop().unwrap() == "yea") || res == None  {
                                match &*current_operator {
                                    "!#" => {
                                        let mem = self.memory.get_mut(&current_memory).unwrap();
                                        let method = String::from(mem.pop().unwrap());
                                        let mut metainterpreter = Ob::new(String::new());
                                        metainterpreter.set(self.memory,self.functions);
                                        metainterpreter.interpret(method);
                                        let (n,o) = metainterpreter.ret();

                                        self.set(n,o);
                                    },

                                    "+" => {
                                        let mem = self.memory.get_mut(&current_memory).unwrap();
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
                                        let mem = self.memory.get_mut(&current_memory).unwrap();
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
                                        let mem = self.memory.get_mut(&current_memory).unwrap();
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
                                        let mem = self.memory.get_mut(&current_memory).unwrap();
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
                                        let mem = self.memory.get_mut(&current_memory).unwrap();
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
                                        let mem = self.memory.get_mut(&current_memory).unwrap();
                                        let thing = mem.pop().unwrap();
                                        println!("{:?}",thing);
                                    },

                                    "goto" => {
                                        println!("at the goto op! {:?}",self.memory[&current_memory]);

                                        let mem = self.memory.get_mut(&current_memory).unwrap();
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
                                        let mem = self.memory.get_mut(&current_memory).unwrap();
                                        let val: &str = &*mem.pop().unwrap();
                                        mem.push(String::from(val));
                                        mem.push(String::from(val));
                                    },

                                    "<=" => {
                                        let first_mem_name = String::from(self.memory
                                            .get_mut(&current_memory)
                                            .unwrap()
                                            .pop()
                                            .unwrap());
                                        let second_mem_name = String::from(self.memory
                                            .get_mut(&current_memory)
                                            .unwrap()
                                            .pop()
                                            .unwrap());
                                        let val = self.memory
                                            .get_mut(&first_mem_name)
                                            .unwrap()
                                            .pop()
                                            .unwrap();
                                        self.memory
                                            .get_mut(&second_mem_name)
                                            .unwrap()
                                            .push(
                                                String::from(
                                                        val
                                                    )
                                                );
                                    },
                                    // self.functions[function_name] is the same thing, just going the opposite direction
                                    "=>" => {
                                        println!("swapping!");
                                        let second_mem_name = String::from(self.memory
                                            .get_mut(&current_memory)
                                            .unwrap()
                                            .pop()
                                            .unwrap());
                                        let first_mem_name = String::from(self.memory
                                            .get_mut(&current_memory)
                                            .unwrap()
                                            .pop()
                                            .unwrap());
                                        let val = self.memory
                                            .get_mut(&first_mem_name)
                                            .unwrap()
                                            .pop()
                                            .unwrap();
                                        self.memory
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
                                        println!("contents of stack {}:{:?}",current_memory,self.memory.get_mut(&current_memory).unwrap());
                                        let new = String::from(&*self.memory.get_mut(&current_memory).unwrap().pop().unwrap());
                                        let mut possible = false;
                                        for a in self.memory.keys(){
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
                                        let name = String::from(&*self.memory.get_mut(&current_memory).unwrap().pop().unwrap());
                                        self.memory.insert(name,Vec::new());
                                    },

                                    "input" => {
                                        let mut line = String::new();
                                        match io::stdin().read_line(&mut line) {
                                            Ok(_) => {();},
                                            Err(_) => {panic!("error reading from stdin!");}
                                        }
                                        self.memory.get_mut(&current_memory).unwrap().push(line);

                                    },

                                    "<" => {
                                        // if there is an option that's a string, we're going to make it a number dammit
                                        let val1 = String::from(self.memory.get_mut(&current_memory).unwrap().pop().unwrap());
                                        let val2 = String::from(self.memory.get_mut(&current_memory).unwrap().pop().unwrap());
                                        match (val1.parse::<i64>(), val2.parse::<i64>()) {
                                            (Ok(x),Ok(y)) => {
                                                if y < x {
                                                    self.memory.get_mut(&current_memory).unwrap().push(String::from("yea"));
                                                }
                                                else{
                                                    self.memory.get_mut(&current_memory).unwrap().push(String::from("nope"));
                                                }
                                            },
                                            // they're trying to judge a number vs a string, then use the length of the string
                                            (Err(_),Ok(y)) => {
                                                if y < len(val1){
                                                    self.memory.get_mut(&current_memory).unwrap().push(String::from("yea"));
                                                }
                                                else{
                                                    self.memory.get_mut(&current_memory).unwrap().push(String::from("nope"));
                                                }
                                            },

                                            (Ok(x),Err(_)) => {
                                                if x < len(val2){
                                                    self.memory.get_mut(&current_memory).unwrap().push(String::from("yea"));
                                                }
                                                else{
                                                    self.memory.get_mut(&current_memory).unwrap().push(String::from("nope"));
                                                }
                                            },

                                            (Err(_),Err(_)) => {
                                                if len(val2) < len(val1){
                                                    self.memory.get_mut(&current_memory).unwrap().push(String::from("yea"));
                                                }
                                                else{
                                                    self.memory.get_mut(&current_memory).unwrap().push(String::from("nope"));
                                                }
                                            }
                                        }
                                        println!("{:?}",self.memory.get(&current_memory));
                                    },

                                    ">" => {
                                        // if there is an option that's a string, we're going to make it a number dammit
                                        let val2 = String::from(self.memory.get_mut(&current_memory).unwrap().pop().unwrap());
                                        let val1 = String::from(self.memory.get_mut(&current_memory).unwrap().pop().unwrap());
                                        match (val1.parse::<i64>(), val2.parse::<i64>()) {
                                            (Ok(x),Ok(y)) => {
                                                if y < x {
                                                    self.memory.get_mut(&current_memory).unwrap().push(String::from("yea"));
                                                }
                                                else{
                                                    self.memory.get_mut(&current_memory).unwrap().push(String::from("nope"));
                                                }
                                            },
                                            // they're trying to judge a number vs a string, then use the length of the string
                                            (Err(_),Ok(y)) => {
                                                if y < len(val1){
                                                    self.memory.get_mut(&current_memory).unwrap().push(String::from("yea"));
                                                }
                                                else{
                                                    self.memory.get_mut(&current_memory).unwrap().push(String::from("nope"));
                                                }
                                            },

                                            (Ok(x),Err(_)) => {
                                                if x < len(val2){
                                                    self.memory.get_mut(&current_memory).unwrap().push(String::from("yea"));
                                                }
                                                else{
                                                    self.memory.get_mut(&current_memory).unwrap().push(String::from("nope"));
                                                }
                                            },

                                            (Err(_),Err(_)) => {
                                                if len(val2) < len(val1){
                                                    self.memory.get_mut(&current_memory).unwrap().push(String::from("yea"));
                                                }
                                                else{
                                                    self.memory.get_mut(&current_memory).unwrap().push(String::from("nope"));
                                                }
                                            }
                                        }
                                        println!("{:?}",self.memory.get(&current_memory));
                                    },

                                    "==" =>{
                                        let val1 = String::from(self.memory.get_mut(&current_memory).unwrap().pop().unwrap());
                                        let val2 = String::from(self.memory.get_mut(&current_memory).unwrap().pop().unwrap());

                                        match (val1.parse::<i64>(), val2.parse::<i64>()) {
                                            (Ok(x),Ok(y)) => {
                                                if y == x {
                                                    self.memory.get_mut(&current_memory).unwrap().push(String::from("yea"));
                                                }
                                                else{
                                                    self.memory.get_mut(&current_memory).unwrap().push(String::from("nope"));
                                                }
                                            },
                                            // they're trying to judge a number vs a string, then use the length of the string
                                            (Err(_),Ok(y)) => {
                                                if y == len(val1){
                                                    self.memory.get_mut(&current_memory).unwrap().push(String::from("yea"));
                                                }
                                                else{
                                                    self.memory.get_mut(&current_memory).unwrap().push(String::from("nope"));
                                                }
                                            },

                                            (Ok(x),Err(_)) => {
                                                if x == len(val2){
                                                    self.memory.get_mut(&current_memory).unwrap().push(String::from("yea"));
                                                }
                                                else{
                                                    self.memory.get_mut(&current_memory).unwrap().push(String::from("nope"));
                                                }
                                            },

                                            (Err(_),Err(_)) => {
                                                if val2 == val1{
                                                    self.memory.get_mut(&current_memory).unwrap().push(String::from("yea"));
                                                }
                                                else{
                                                    self.memory.get_mut(&current_memory).unwrap().push(String::from("nope"));
                                                }
                                            }
                                        }
                                    },

                                    "!" => {
                                        match &*self.memory.get_mut(&current_memory).unwrap().pop().unwrap(){
                                            "yea" => {
                                                self.memory.get_mut(&current_memory).unwrap().push(String::from("nope"));
                                            },
                                            "nope" => {
                                                self.memory.get_mut(&current_memory).unwrap().push(String::from("yea"));
                                            },

                                            _ => ()
                                        }
                                    }

                                    _ => {
                                        println!("pushed {:?} onto the stack {}",current_operator,current_memory);
                                        self.memory.get_mut(&current_memory).unwrap().push(String::from(current_operator));
                                    }
                                }
                            }
                        }
                    }
                }
            }
            count += 1;
            // println!("{:?}",self.memory["main"]);
        }
    }
}
