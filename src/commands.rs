use std::process;
use std::{io::{self, Write}, str::FromStr};
use std::env;
use std::path::Path;

//PREAMBLE
enum TokenType {
    ECHO,
    EXIT,
    TYPE,
    PWD,
    CD,
}

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

//Command functions

//External commands

//Builtins
pub fn echo(arg: &Vec<String>){
    println!("{}", arg.join(" "));
}

pub fn exit(){
    process::exit(0);
}

pub fn type_check(arg: &Vec<String>){
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

/* 
fn pwd(){
    let working_dir  = std::env::current_dir();
    println!("{}", working_dir.display());
    Ok(())
}

fn cd(arg: &Vec<String>){

}

*/
