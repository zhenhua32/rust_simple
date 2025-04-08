use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust: 
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn test_build() {
        let args = vec![
            String::from("minigrep"),
            String::from("duct tape"),
            String::from("poem.txt"),
        ];
        let config = Config::build(&args).unwrap();
        assert_eq!(config.query, "duct tape");
        assert_eq!(config.filename, "poem.txt");
    }

    #[test]
    fn test_not_enough_arguments() {
        let args = vec![String::from("minigrep")];
        let result = Config::build(&args);
        assert!(result.is_err());
    }
}
