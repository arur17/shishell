//use std::io;
use std::io::{self, Write};
use std::process::{Command, Stdio, Child};
use std::env;
use std::path::Path;

fn main () {
   loop{
        print!(">"); // it will be used as a character prompt. 
        io::stdout().flush();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // //everything after the first whitespace character is interpreted as args to the command 
        // let mut parts = input.trim().split_whitespace();
        // let command = parts.next().unwrap(); //to remove the newline from read_line
        // let args = parts;

        //Must be peekable so we know when we are on the last command.
        let mut commands = input.trim().split (" | ").peekable();
        let mut previous_command = None;

    

        // let mut child = Command::new(command)
        //     .args(args)
        //     .spawn()
        //     .unwrap();

        // child.wait(); 
        // //do not accept new command until this one is done

        while let Some(Command) = commands.next() {

            let mut parts = Command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;
            match command {
                "cd" => {
                    // default to '/' as the new dir if one was not provided
                    let new_dir = args.peekable().peek().map_or("/", |x| *x);
                    let root = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }
                    previous_command = None;
                },
                "exit" => return,

                command => {
                    // let mut child = Command::new(command)
                    //     .args(args)
                    //     .spawn();

                    // //Gracefully handle malformed user input
                    // match child {
                    //     Ok(mut child) => {child.wait();},
                    //     Err(e) => eprintln!("{}", e),
                    // };

                    let stdin = previous_command
                        .map_or(
                            Stdio::inherit(),
                            |output: Child| Stdio::from(output.stdout.unwrap())
                        );
                    let stdout = if commands.peek().is_some(){
                        //there is another command piped behind this one
                        //prepare to send output to the next command 
                        Stdio::piped()
                    } else {
                        //there are no more commands piped behind this one
                        //send out put to the shell stdout
                        Stdio::inherit()
                    };

                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();
                    match output {
                        Ok(output) => { previous_command = Some(output);},
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}", e);
                        },
                    };
                }
            }
        }
        if let Some(mut final_command) = previous_command {
            //block until final command is finished
            final_command.wait();
        }
   }
}



