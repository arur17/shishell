//use std::io;
use std::io::{self, Write};
use std::process::Command;
use std::env;
use std::path::Path;

fn main () {
   loop{
        print!(">"); // it will be used as a character prompt. 
        io::stdout().flush();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        //everything after the first whitespace character is interpreted as args to the command 
        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap(); //to remove the newline from read_line
        let args = parts;

        // let mut child = Command::new(command)
        //     .args(args)
        //     .spawn()
        //     .unwrap();

        // child.wait(); 
        // //do not accept new command until this one is done

        match command {
            "cd" => {
                // default to '/' as the new dir if one was not provided
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            },
            "exit" => return,

            command => {
                let mut child = Command::new(command)
                    .args(args)
                    .spawn();

                //Gracefully handle malformed user input
                match child {
                    Ok(mut child) => {child.wait();},
                    Err(e) => eprintln!("{}", e),
                };
            }
        }
   }
}



