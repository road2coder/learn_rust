use std::{env, error::Error, fs};

#[derive(Debug)]
pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        let ignore_case = env::var("IGNORE_CASE").unwrap_or_default();
        let ignore_case = ignore_case == "1" || ignore_case == "true";
        let query = args.next().ok_or("Did not get a query string")?;
        let file_path = args.next().ok_or("Did not get a file path")?;
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for line in results {
        println!("{line}");
    }
    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_empty() {
        let query = "Duct";
        let contents = "\
            Rust:\n\
            safe, fast, productive\n\
            Pick three.";
        assert!(search(query, contents).is_empty())
    }

    #[test]
    fn one_result() {
        let query = "safe";
        let contents = "\
            Rust:\n\
            safe, fast, productive\n\
            Pick three.";
        assert_eq!(vec!["safe, fast, productive"], search(query, contents));
    }
}
