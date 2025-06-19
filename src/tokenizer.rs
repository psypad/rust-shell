use regex::Regex;
use crate::lexer::lexer;

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

pub fn tokenizer(mut input: String){

    trim_newline(&mut input);
    println!("DEBUG (input string from tokenizer): {:?}", input);

    let mut arg_arr: Vec<String> = Vec::new();

    let text = input.to_owned();
    let re = Regex::new(r#""[^"]*"|[|<>]|\S+"#).unwrap();

    for s in re.captures_iter(text.as_str()){    
            arg_arr.push(s.get(0).unwrap().as_str().to_string());
    }

    println!("DEBUG (tokenized string from tokenizer): {:?}", arg_arr);

    lexer(arg_arr);
}
