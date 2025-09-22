use jgrep::{search, search_insensitive};
use std::error::Error;
use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    };
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.insensitive {
        search_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

struct Config {
    query: String,
    file_path: String,
    insensitive: bool,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        let insensitive: bool;
        let query: String;
        let file_path: String;

        if args.len() < 3 {
            return Err("not enough arguments");
        } else if args.len() > 4 {
            return Err("too many arguments");
        } else if args.len() == 3 {
            insensitive = false;
            query = args[1].clone();
            file_path = args[2].clone();
        } else {
            if args[1] != "--insensitive" {
                return Err("option not recongnized");
            }
            insensitive = true;
            query = args[2].clone();
            file_path = args[3].clone();
        }

        Ok(Config {
            query,
            file_path,
            insensitive,
        })
    }
}
