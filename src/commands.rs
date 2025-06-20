use std::process;
use std::{io::{self,BufRead}, str::FromStr};
use std::env;
use std::path::Path;
use std::fs;
use regex::Regex;
use std::fs::File;

//PREAMBLE
enum TokenType {
    ECHO,
    EXIT,
    TYPE,
    PWD,
    CD,
    CAT,
    GREP,
    LS,
}

impl FromStr for TokenType{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str(){
            "echo" => Ok(TokenType::ECHO),
            "type" => Ok(TokenType::TYPE),
            "exit" => Ok(TokenType::EXIT),
            "pwd" => Ok(TokenType::PWD),
            "cd" => Ok(TokenType::CD),
            "cat" => Ok(TokenType::CAT),
            "grep" => Ok(TokenType::GREP),
            "ls" => Ok(TokenType::LS),
            _=> Err(()),
        }
    }
}

//Command functions

//External commands

//Builtins
pub fn echo(arg: &Vec<String>){
    println!("{}", arg.join(" "));
}

pub fn exit(){
    process::exit(0);
}

pub fn type_check(arg: &Vec<String>){
        match TokenType::from_str(&arg[0]){
        Ok(TokenType::ECHO) => println!("echo is a shell builtin"),
        Ok(TokenType::EXIT) => println!("exit is a shell builtin"),
        Ok(TokenType::TYPE) => println!("type is a shell builtin"),
        Ok(TokenType::PWD) => println!("pwd is a shell builtin"),
        Ok(TokenType::CD) => println!("cd is a shell builtin"),
        Ok(TokenType::GREP) => println!("grep is a shell builtin"),
        Ok(TokenType::CAT) => println!("cat is a shell builtin"),
        Ok(TokenType::LS) => println!("ls is a shell builtin"),
        _ => {
            println!("command {} not found", &arg[0]);
        }
    }
}

pub fn cd(args: &Vec<String>) {
    if args.is_empty() {
        match env::var("HOME") {
            Ok(home) => {
                if let Err(e) = env::set_current_dir(home) {
                    eprintln!("cd: {}", e);
                }
            },
            Err(_) => eprintln!("cd: HOME not set"),
        }
    } else {
        if let Err(e) = env::set_current_dir(&args[0]) {
            eprintln!("cd: {}: {}", args[0], e);
        }
    }
}

pub fn pwd() -> io::Result<()> {
    let dir = env::current_dir()?;
    println!("{}", dir.display());
    Ok(())
}

pub fn cat(args: &Vec<String>) {
    if args.is_empty() {
        eprintln!("cat: missing file operand");
        return;
    }

    for filename in args {
        match fs::read_to_string(filename) {
            Ok(content) => print!("{}", content),
            Err(e) => eprintln!("cat: {}: {}", filename, e),
        }
    }
}

pub fn grep(args: &Vec<String>) {
    if args.len() < 2 {
        eprintln!("grep: usage: grep <pattern> <file>");
        return;
    }

    let pattern = match Regex::new(&args[0]) {
        Ok(re) => re,
        Err(e) => {
            eprintln!("grep: invalid regex '{}': {}", args[0], e);
            return;
        }
    };

    let file = File::open(&args[1]);
    let reader = match file {
        Ok(f) => io::BufReader::new(f),
        Err(e) => {
            eprintln!("grep: {}: {}", args[1], e);
            return;
        }
    };

    for line in reader.lines() {
        if let Ok(l) = line {
            if pattern.is_match(&l) {
                println!("{}", l);
            }
        }
    }
}

pub fn ls(args: &Vec<String>) {
    let path = if args.is_empty() {
        "."
    } else {
        &args[0]
    };

    let entries = fs::read_dir(Path::new(path));
    match entries {
        Ok(dir_iter) => {
            for entry in dir_iter {
                if let Ok(entry) = entry {
                    if let Ok(name) = entry.file_name().into_string() {
                        println!("{}", name);
                    }
                }
            }
        },
        Err(e) => eprintln!("ls: {}: {}", path, e),
    }
}
