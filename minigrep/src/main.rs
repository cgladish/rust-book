use std::{env, fs, process, error::Error};

use minigrep::search;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Failed to load config: {err}");
        process::exit(1);
    });

    println!("Search for {}", config.query);
    println!("In filepath {}", config.filepath);

    if let Err(err) = run(config) {
        eprintln!("Failed when running: {err}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filepath)?;

    let result = search(config.query, &contents);
    for line in &result {
        println!("{line}");
    }

    Ok(())
}

struct Config<'a> {
    query: &'a String,
    filepath: &'a String
}

impl<'a> Config<'a> {
    fn build(args: &'a Vec<String>) -> Result<Config<'a>, &'static str> {
        if args.len() < 3 {
            return Err("Missing query and filepath args");
        }

        let query = &args[1];
        let filepath = &args[2];
        
        Ok(Config { query, filepath })
    }
}