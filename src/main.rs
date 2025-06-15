mod lexer;
mod parser;
mod commands;

use std::{io::{self, Write}, str::FromStr};
use std::process;
use std::env;
use std::path::Path;
use lexer::lexer;

//PREAMBLE
enum TokenType {
    ECHO,
    EXIT,
    TYPE,
    PWD,
    CD,
}

// converting from string to type TokenType 
// check if a given keyword is enumerated
impl FromStr for TokenType{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str(){
            "echo" => Ok(TokenType::ECHO),
            "type" => Ok(TokenType::TYPE),
            "exit" => Ok(TokenType::EXIT),
            "pwd" => Ok(TokenType::PWD),
            "cd" => Ok(TokenType::CD),
            _=> Err(()),
        }
    }
}

struct Command{
    keyword: String,
    arguments: Vec<String>
}

fn print_prompt(){
        print!("$ ");
        io::stdout().flush().unwrap();
}

// takes input string -> builds token array -> then appends it to a token struct
fn token_struct_builder(input: String) -> Command{

        let mut token_array: Vec<String> = vec![];
        let mut current_token = String::new();

        for c in input.chars() {
            if c != ' ' && c != '\n' && c != '\r' {
                current_token.push(c);
            } else {
                if !current_token.is_empty() {
                    token_array.push(current_token.clone());
                    current_token.clear();
                }
            }
        }

        if !current_token.is_empty() {
            token_array.push(current_token);
        }

        println!("DEBUG (tokenized array): {:?}", token_array);

        Command{
            keyword:token_array.remove(0),
            arguments:token_array
        }

}

// gives the keywords function 
fn define_function(command : Command){
    match TokenType::from_str(&command.keyword){
        Ok(TokenType::ECHO) => echo(&command.arguments),
        Ok(TokenType::EXIT) => exit(),
        Ok(TokenType::TYPE) => type_check(&command.arguments),
        Ok(TokenType::PWD) => pwd().expect("might panic: dunno why cause it just has to print the working dir"),
        Ok(TokenType::CD) => cd(&command.arguments),
        _ => {
            println!("command {} not found", &command.keyword);
        }
    }
}

//Newamble

//-----moved the new lexer to its own file 

//PREAMBLE END
//------------------------------------------------------------

//Command functions

//Builtins
fn echo(arg: &Vec<String>){
    println!("{}", arg.join(" "));
}

fn exit(){
    process::exit(0);
}

fn type_check(arg: &Vec<String>){
    match TokenType::from_str(&arg[0]){
        Ok(TokenType::ECHO) => println!("echo is a shell builtin"),
        Ok(TokenType::EXIT) => println!("exit is a shell builtin"),
        Ok(TokenType::TYPE) => println!("type is a shell builtin"),
        Ok(TokenType::PWD) => println!("pwd is a shell builtin"),
        Ok(TokenType::CD) => println!("cd is a shell builtin"),
        _ => {
            println!("command {} not found", &arg[0]);
        }
    }
}

fn pwd() -> std::io::Result<()>{
    let working_dir  = env::current_dir()?;
    println!("{}", working_dir.display());
    Ok(())
}

fn cd(arg: &Vec<String>){

}

fn pipe_behaviour(){
    
}

//External commands

//-------------
//Main function
fn main() {
    let stdin = io::stdin();
    
    loop {
        print_prompt();

        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        
        print!("DEBUG (raw input): {}", input);

        let tokenprinter  = lexer(input);
        
        //let token_struct: Command = token_struct_builder(input.clone());

        println!("{:?}",tokenprinter);
        //println!("DEBUG (Command struct): {} {:?}", token_struct.keyword, token_struct.arguments);

       //define_function(token_struct);

    }
}

//tests
#[cfg(test)]

fn first_test(){

}