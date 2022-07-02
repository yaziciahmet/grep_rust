use std::fs;
use std::error::Error;
use std::env;


pub struct Config {
  pub query: String,
  pub filename: String,
  pub ignore_case: bool,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
      if args.len() != 3 {
          return Err("Not enough arguments");
      }
      let query = args[1].clone();
      let filename = args[2].clone();
      let ignore_case = env::var("IGNORE_CASE").is_ok();

      Ok(Config { query, filename, ignore_case })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.filename)?;

  let search = if config.ignore_case { search_case_insensitive } else { search_case_sensitive };

  for line in search(&config.query, &contents) {
    println!("{}", line);
  }

  Ok(())
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut result = vec![];

  for line in contents.lines() {
    if line.contains(query) {
      result.push(line);
    }
  }

  result
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut result = vec![];
  let query = query.to_lowercase();

  for line in contents.lines() {
    if line.to_lowercase().contains(&query) {
      result.push(line);
    }
  }

  result
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";

    assert_eq!(vec!["safe, fast, productive."], search_case_sensitive(query, contents));
  }

  #[test]
  fn case_insensitive() {
    let query = "rUSt";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
  }
}