use std::process;

//Command functions

//External commands

//Builtins
fn echo(arg: &Vec<String>){
    println!("{}", arg.join(" "));
}

fn exit(){
    process::exit(0);
}

fn type_check(arg: &Vec<String>){

}

fn pwd(){

}

fn cd(arg: &Vec<String>){

}

fn pipe_behaviour(){
    
}
