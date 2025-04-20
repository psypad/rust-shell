use std::collections::HashMap;
use std::io::{self, Write};

enum TokenType {
    ECHO,
    EXIT,
    TYPE,
}

fn check_token(s:String){
    let mut keywords: HashMap<&str, TokenType> = HashMap::new();

    keywords.insert("echo", TokenType::ECHO);
    keywords.insert("exit", TokenType::EXIT);
    keywords.insert("type", TokenType::TYPE);

}




fn main() {
    let stdin = io::stdin();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let char_vec: Vec<char> = input.chars().collect();


        let mut current: i32 = 0;
        let mut token_array: Vec<String> = vec![];
            
        for c in char_vec {
            println!("{}", c);
            
            let mut s = String::new();
    
            if c != ' '{
                s.push(c);
                current += 1;
                
            } else {
                //let  s: String = token.into_iter().collect();
                println!("{}", s);
                token_array.push(s);
                println!("inside else")
            }
        }      

        println!("{:?}",token_array);

        /* 
        match input.trim() {
            "exit 0" => break,
            input if input.starts_with("echo ") => println!("{}", &input[5..]),
            input if input.starts_with("type ") => match &input[5..] {
                "echo" | "exit" | "type" => println!("{} is a shell builtin", &input[5..]),
                _ => println!("{}: not found", &input[5..]),
            },
            _ => println!("{}: command not found", input.trim()),
        }
        */
    }
}