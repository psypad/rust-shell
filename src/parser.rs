use crate::lexer::{self, TokenTypesAdvanced};
use crate::executor::executor;

#[derive(Debug)]
pub enum ControlFlow{
    None,
    Pipe(Box<CommandNode>),
    RedirectOut(Box<CommandNode>),
    RedirectIn(Box<CommandNode>),
}

#[derive(Debug)]
pub struct CommandNode{
    pub keyword: String,
    pub argument: Vec<String>,
    pub control_flow: ControlFlow,
}

pub fn parse_from(index: &mut usize ,tokens: &Vec<TokenTypesAdvanced>) -> CommandNode{
    

    let mut keyword = String::new();
    let mut arguments = vec![];
    let mut control_flow = ControlFlow::None;

    while *index < tokens.len() {    
        if keyword.is_empty() {
            match &tokens[*index] {
                TokenTypesAdvanced::Keyword(k) => {
                    keyword = k.clone();
                    *index += 1;                    
                },
                _ => {}
            }
        } else {
            match &tokens[*index] {
                TokenTypesAdvanced::Argument(a) => {
                    arguments.push(a.clone());
                    *index += 1;
                },
                TokenTypesAdvanced::Keyword(k) => {
                    arguments.push(k.clone()); 
                    *index += 1;
                },
                TokenTypesAdvanced::Option(o) => {
                    arguments.push(o.clone());
                    *index += 1;
                },
                TokenTypesAdvanced::Pipe => {
                    *index += 1;
                    control_flow = ControlFlow::Pipe(Box::new(parse_from(index, tokens)));
                    break;
                },
                TokenTypesAdvanced::RedirectOut => {
                    *index += 1;
                    control_flow = ControlFlow::RedirectOut(Box::new(parse_from(index, tokens)));
                    break;
                },
                TokenTypesAdvanced::RedirectIn => {
                    *index += 1;
                    control_flow = ControlFlow::RedirectIn(Box::new(parse_from(index, tokens)));
                    break;
                },
                _ => { *index += 1;}  
            } 
        }
    }

    CommandNode {
        keyword: keyword,
        argument: arguments,
        control_flow,
    }
}


pub fn parser(token_type_vec: Vec<TokenTypesAdvanced>) {
    let mut index = 0;
    let ast = parse_from(&mut index, &token_type_vec);

    println!("DEBUG (AST root): {:?}", ast); 
    
    executor(ast);
}