use crate::parser::{self,CommandNode};
use crate::commands::{echo};

pub fn executor(command: CommandNode){
    match command.keyword.as_str(){
        "echo" => echo(&command.argument),/* 
        "exit" =>
        "type" => 
        "cd"   =>
        "pwd"  =>
        "cat" | "grep" | "ls" => 
        */
        _ => ()
    }
}