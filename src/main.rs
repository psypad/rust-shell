mod lexer;
mod parser;
mod commands;
mod tokenizer;

use std::{io::{self, Write}};

use crate::tokenizer::tokenizer;

fn print_prompt(){
        print!("$ ");
        io::stdout().flush().unwrap();
}

fn main() {
    let stdin = io::stdin();
    
    loop {
        print_prompt();

        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        
        print!("DEBUG (raw input): {}", input);

        tokenizer(input);
    }
}



//-----------------------------------STORAGE----------------------------------------------------------

//let token_struct: Command = token_struct_builder(input.clone());

//println!("DEBUG (Command struct): {} {:?}", token_struct.keyword, token_struct.arguments);

//define_function(token_struct);

//PREAMBLE


//Newamble

//-----moved the new lexer to its own file----

//PREAMBLE END
//------------------------------------------------------------

//-----moved the commands to their own file(future refactor each command gets thier own file)----

//-------------
//Main function
