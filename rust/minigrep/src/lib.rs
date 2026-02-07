use std::env;
use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let mut query: Option<String> = None;
        let mut file_path: Option<String> = None;
        let mut ignore_case = env::var("IGNORE_CASE").is_ok();

        for arg in args {
            match arg.as_str() {
                "--ignore-case" | "-i" | "-ignore" => {
                    ignore_case = true;
                }
                flag if flag.starts_with("--") => {
                    return Err("Unknown flag detected");
                }
                _ => {
                    if query.is_none() {
                        query = Some(arg); // 填充第一个空位
                    } else if file_path.is_none() {
                        file_path = Some(arg); // 填充第二个空位
                    } else {
                        return Err("Too many arguments supplied");
                    }
                }
            }
        }

        let query = query.ok_or("Missing query argument")?;
        let file_path = file_path.ok_or("Missing file path argument")?;

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

// in lib.rs
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
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
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
