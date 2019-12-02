use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String], case_sensitive: bool) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone(),
            case_sensitive,
        })
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.filename)?;

    for line in search(&config.query, &content, config.case_sensitive) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str, case_sensitive: bool) -> Vec<&'a str> {
    let mut results = Vec::new();

    if !case_sensitive {
        let query = query.to_lowercase();
        for line in content.lines() {
            if line.to_lowercase().contains(&query) {
                results.push(line)
            }
        }

        return results;
    }

    for line in content.lines() {
        if line.contains(query) {
            results.push(line)
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

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents, true)
        );
    }

    #[test]
    fn three_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Product is always good.
Product is always best.";

        assert_eq!(
            vec![
                "safe, fast, productive.",
                "Product is always good.",
                "Product is always best."
            ],
            search(query, contents, true)
        );
    }

    #[test]
    fn three_result_case_insensitive() {
        let query = "dUcT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Product is always good.
Product is always best.";

        assert_eq!(
            vec![
                "safe, fast, productive.",
                "Product is always good.",
                "Product is always best."
            ],
            search(query, contents, false)
        );
    }

    #[test]
    fn one_result_case_insensitive() {
        let query = "dUcT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents, false)
        );
    }
}
