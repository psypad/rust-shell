use crate::parser::{self,CommandNode,ControlFlow};
use crate::commands::{echo, exit, type_check};

pub fn executor(command: CommandNode){
    match command.keyword.as_str(){
        "echo" => echo(&command.argument), 
        "exit" => exit(),
        "type" => type_check(&command.argument),
        /*
        "cd"   =>
        "pwd"  =>
        "cat" | "grep" | "ls" => 
        */
        _ => ()
    }

    match command.control_flow {
        ControlFlow::None => {},
        ControlFlow::Pipe(next) => {
            println!("pipe → {:?}", next.keyword); 
            executor(*next); 
        },
        ControlFlow::RedirectOut(next) => {
            println!("redirect stdout → {:?}", next.keyword);
            executor(*next);
        },
        ControlFlow::RedirectIn(next) => {
            println!("redirect stdin ← {:?}", next.keyword);
            executor(*next);
        },
    }
}