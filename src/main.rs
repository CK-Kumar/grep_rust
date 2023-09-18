use std::env;
use std::error::Error;
use std::fs;
use std::process;
use std::error;
fn main(){
let args: Vec<String> = env::args().collect();
let config = Config::new(&args).unwrap_or_else(
    |err|{
        println!("{}", err);
        process::exit(1);
    }
);

println!("Searching for : {}", config.query );
println!("In File : {}", config.filename);
if let Err(e) = run(config)  {
    println!("Application Error: {} ", e);
    process::exit(1);
}
}

fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let content = fs::read_to_string(config.filename).expect("Error reading the file");
    println!("Content: {} \n", content);
    Ok(())
}

struct Config{
    query:String,
    filename:String
}

impl Config {
    fn new(arg:&[String])->Result<Config, &str> {
        if arg.len()<3
        {
            return Err("Not enough Arguments");
        }
        let query = arg[1].clone();
        let filename = arg[2].clone();
        Ok(Config { query: query, filename: filename })
    }

}