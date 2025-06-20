use crate::parser::parser;


#[derive(Debug)]
pub enum TokenTypesAdvanced{
    Keyword(String),
    Argument(String),
    Option(String),
    Pipe,
    RedirectOut,
    RedirectIn,
}

pub fn lexer(tokenized: Vec<String>) {
    let mut token_type_vec:Vec<TokenTypesAdvanced> = Vec::new();

    for part in tokenized{
        match part.as_str(){
            "|" => token_type_vec.push(TokenTypesAdvanced::Pipe),
            ">" => token_type_vec.push(TokenTypesAdvanced::RedirectOut),
            "<" => token_type_vec.push(TokenTypesAdvanced::RedirectIn),
            "-" | "--" =>{ 
                if part.starts_with("--") || part.starts_with("-") {
                    token_type_vec.push(TokenTypesAdvanced::Option(part.to_string()));
                }else {
                    token_type_vec.push(TokenTypesAdvanced::Option(part.to_string()));
                }
            },
            "echo" | "exit" | "type" | "cd" | "pwd" | "cat" | "grep" | "ls" | "sudo" => token_type_vec.push(TokenTypesAdvanced::Keyword(part.to_string())),
            _ => token_type_vec.push(TokenTypesAdvanced::Argument(part.to_string())),
        }
    }

    println!("DEBUG (lexed array from lexer): {:?}",token_type_vec);
    parser(token_type_vec);     
}
