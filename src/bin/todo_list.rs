use std::env;
use std::fs;
use std::process;
use std::error::Error;

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new (args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query: String = args[1].clone();
        let file_path: String = args[2].clone();
        Ok (
            Config { query, file_path }
        )
    }
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
        });

    println!("Query: {}", config.query);
    println!("File path: {}", config.file_path);

    let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    };

}

fn run (config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    println!("{}", contents);

    Ok(())
}

