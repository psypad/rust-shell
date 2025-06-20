use crate::parser::{CommandNode,ControlFlow};
use crate::commands::{echo, exit, type_check,cd,cat,pwd,grep,ls};

pub fn executor(command: CommandNode){
    match command.keyword.as_str(){
        "echo" => echo(&command.argument), 
        "exit" => exit(),
        "type" => type_check(&command.argument),
        "cd"   => cd(&command.argument),
        "pwd"  => pwd().expect("panic!"),
        "cat"  =>cat(&command.argument),
        "grep" =>grep(&command.argument),
        "ls"   => ls(&command.argument),
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