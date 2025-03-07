use std::io::{self, Write};

fn main() {
    //Uncomment this block to pass the first stage
    //print!("$ ");
    //io::stdout().flush().unwrap();
    let stdin = io::stdin();

    loop {
        //let errcode = 0;
        let mut command_array: Vec<&str>  = vec![]; //= ["echo", "exit", "type"];

        command_array.push("echo");
        command_array.push("exit");
        command_array.push("type");

        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        //print!("{input}");

        

        match input.trim() {
            "exit 0" => break,
            input if input.starts_with("echo ") => println!("{}", &input[5..]),
            input if input.starts_with("type ") => 
            match &input[5..] {
                "echo" | "exit" | "type" => println!("{} is a shell builtin", &input[5..]),
                _ => println!("{}: not found", &input[5..]),
            },
            _ => println!("{}: command not found", input.trim()),
        } 

        /* 
        if input.trim() == "exit 0"{
            //println!("exit {}", errcode);
            break;
        }
        if input.contains("echo") == true && input.contains("type") == false {
            let echoed_word = &input.trim()[5..];
            println!("{}", echoed_word);
        }

        if input.contains("type") == true{
            if input.contains("echo") == true{
                println!("echo is a shell builtin")
            }
            if input.contains("exit") == true{
                println!("exit is a shell builtin")
            }
            if input.contains("type type") == true{
                println!("type is a shell builtin")
            }
        }


        for i in command_array{                         
            if input.contains(i) == true{
                continue;
            }else{
                println!("{}: command not found", input.trim());
            }
        }
        */

    }

}

 

/* 

fn Command_function(x: String) -> Vec<String> {
    let mut command_array = Vec::new();

    command_array.push(x);
    
    return command_array;
}



fn command_picker (command_array: Vec<&str>, entered_command: &str) -> bool{

    for i in command_array{
        if entered_command.contains(&i) == true {
            return true;
        }
    }

    return false;
}


 */