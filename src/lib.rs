
use std::error::Error;
use std::fs;



pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let content = fs::read_to_string(config.filename).expect("Error reading the file");
    println!("Content: {} \n", content);
    Ok(())
}

pub struct Config{
   pub query:String,
   pub filename:String
}

impl Config {
    pub fn new(arg:&[String])->Result<Config, &str> {
        if arg.len()<3
        {
            return Err("Not enough Arguments");
        }
        let query = arg[1].clone();
        let filename = arg[2].clone();
        Ok(Config { query: query, filename: filename })
    }

}