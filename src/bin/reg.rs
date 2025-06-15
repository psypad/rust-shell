    use regex::Regex;

    fn main(){
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
    
    //--------------------------------------------------------
        //println!("alt file runner")
    }