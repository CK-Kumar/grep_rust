use std::env;
use std::error::Error;
use std::fs;
use std::process;
use rustgrep::Config;

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
if let Err(e) = rustgrep::run(config)  {
    println!("Application Error: {} ", e);
    process::exit(1);
}
}



