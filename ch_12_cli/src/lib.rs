use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>>  {
    // Slurp the entire file into 'contents'
    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    // For debugging
    println!("With text:\n{contents}");

    // Results
    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
    /*
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
    */
}

