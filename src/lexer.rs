use regex::Regex;


#[derive(Debug)]
pub enum TokenTypesAdvanced{
    Keyword(String),
    Argument(String),
    Pipe,
    RedirectOut,
    RedirectIn,
}

fn tokenizer(input: String) -> Vec<String>{
    let mut arg_arr: Vec<String> = Vec::new();

    let text = "echo \"hello world\" | type echo | echo hello world2 | echo lmao2";
    let segments: Vec<&str> = text.split('|').map(str::trim).collect();

    println!("{:?}", segments);

    let re = Regex::new(r#""[^"]*"|\S+"#).unwrap();


    for s in segments{
        for arg in re.find_iter(s){
            arg_arr.push(arg.as_str().to_string());
        }
    }

    println!("{:?}", arg_arr);

    println!("{}",arg_arr[1]);

    arg_arr
}

pub fn lexer(input: String) -> Vec<TokenTypesAdvanced>{

    let tokenized =tokenizer(input);

    let mut token_type_vec:Vec<TokenTypesAdvanced> = Vec::new();


    for part in tokenized{
        match part.as_str(){
            "|" => token_type_vec.push(TokenTypesAdvanced::Pipe),
            ">" => token_type_vec.push(TokenTypesAdvanced::RedirectOut),
            "<" => token_type_vec.push(TokenTypesAdvanced::RedirectIn),
            "echo" | "type" | "cd" | "pwd" | "exit" => token_type_vec.push(TokenTypesAdvanced::Keyword(part.to_string())),
            _ => token_type_vec.push(TokenTypesAdvanced::Argument(part.to_string())),
        }
    }

    token_type_vec     
}
