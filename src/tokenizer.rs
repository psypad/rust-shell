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

// TODO add the pipes in the tokenized array

pub fn tokenizer(mut input: String){

    trim_newline(&mut input);
    println!("DEBUG (input string from tokenizer): {:?}", input);


    let mut arg_arr: Vec<String> = Vec::new();

    let text = "echo \"hello world\" | type echo | echo hello world2 | echo lmao2";
    let segments: Vec<&str> = text.split('|').map(str::trim).collect();

    let re = Regex::new(r#""[^"]*"|\S+"#).unwrap();


    for s in segments{
        for arg in re.find_iter(s){
            arg_arr.push(arg.as_str().to_string());
        }
    }

    println!("DEBUG (tokenized string from tokenizer): {:?}", arg_arr);

    lexer(arg_arr);
}
