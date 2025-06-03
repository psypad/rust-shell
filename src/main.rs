use std::{io::{self, Write}, str::FromStr};
use std::process;

enum TokenType {
    ECHO,
    EXIT,
    TYPE,
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

        Command{
            keyword:token_array.remove(0),
            arguments:token_array
        }

}

// gives the keywords function 
fn define_function(command : Command){
    match TokenType::from_str(&command.keyword){
        Ok(TokenType::ECHO) => echo(&command.arguments),
        Ok(TokenType::EXIT) => exit(&command.arguments),
        Ok(TokenType::TYPE) => type_check(&command.arguments),
        _ => {
            println!("command {} not found", &command.keyword);
        }
    }
}

fn echo(arg: &Vec<String>){
    println!("{}", arg.join(" "));
}

fn exit(arg: &Vec<String>){
    process::exit(0);
}

fn type_check(arg: &Vec<String>){
    match TokenType::from_str(&arg[0]){
        Ok(TokenType::ECHO) => println!("echo is a shell builtin"),
        Ok(TokenType::EXIT) => println!("exit is a shell builtin"),
        Ok(TokenType::TYPE) => println!("type is a shell builtin"),
        _ => {
            println!("command {} not found", &arg[0]);
        }
    }
}

fn main() {
    let stdin = io::stdin();
    

    loop {
        print_prompt();

        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        
        let token_struct: Command = token_struct_builder(input.clone());

        print!("DEBUG (raw input): {}", input);
        println!("DEBUG (tokenized array): {} {:?}", token_struct.keyword, token_struct.arguments);

        define_function(token_struct);

    }
}