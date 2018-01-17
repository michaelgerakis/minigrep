use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
  query: String,
  filename: String,
}

impl Config {
  pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
    args.next();

    let query = match args.next() {
      Some(arg) => arg,
      None => return Err("Please specify a query argument"),
    };

    let filename = match args.next() {
      Some(arg) => arg,
      None => return Err("Please specify a filename"),
    };

    Ok(Config { query, filename })
  }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
  let mut file = File::open(config.filename)?;

  let mut contents = String::new();
  file.read_to_string(&mut contents)?;

  for line in search(&config.query, &contents) {
    println!("{}", line);
  }

  Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  contents
    .lines()
    .filter(|line| line.contains(query))
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
}
