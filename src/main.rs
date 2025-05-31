use std::io::{self, Write};


enum TokenType {
    ECHO,
    EXIT,
    TYPE,
}

fn token_array_builder(input: String) -> Vec<String>{

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

        return token_array;
}

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
        
        let token_array: Vec<String> = token_array_builder(input.clone());

        println!("DEBUG (raw input): {}", input);
        println!("DEBUG (tokenized array): {:?}",token_array);

        match &token_array[0] {
            //"exit 0" => break,
            //"echo".to_owned() => println!("{}", &input[5..]),
            _ => println!(": command not found"),
        }
        
    }

}