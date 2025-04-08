use std::env;
use std::fs;
use std::process;

// 启动命令: cargo run -- "duct tape" poem.txt
fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename).expect("Something went wrong reading the file");
    println!("With text:\n{}", contents);

    println!("Hello, world!");
}

struct Config {
    query: String,
    filename: String,
}

impl Config{
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        } else if args.len() > 3 {
            return Err("Too many arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

fn parse_config(args: &[String]) -> Config {
    if args.len() < 3 {
        panic!("Not enough arguments");
    } else if args.len() > 3 {
        panic!("Too many arguments");
    }

    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}
