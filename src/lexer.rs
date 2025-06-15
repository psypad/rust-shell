#[derive(Debug)]
pub enum TokenTypesAdvanced{
    Keyword(String),
    Argument(String),
    Pipe,
    RedirectOut,
    RedirectIn,
}

pub fn lexer(input: String) -> Vec<TokenTypesAdvanced>{
    let mut tokenized_string:Vec<TokenTypesAdvanced> = Vec::new();

    for part in input.split_whitespace(){
        match part{
            "|" => tokenized_string.push(TokenTypesAdvanced::Pipe),
            _ => tokenized_string.push(TokenTypesAdvanced::Argument(part.to_string())),
        }
    }

    tokenized_string     
}
