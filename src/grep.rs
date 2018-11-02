use std::error::Error;
use std::fs;

pub struct Config {
  pub query: String,
  pub filename: String,
  pub case_sensitive: bool
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
      Err("not enough parameters")
    } else {
      let query = args[1].clone();
      let filename = args[2].clone();
      let case_sensitive = std::env::var("RUST_CASE_SENSITIVE").is_err();
      Ok(Config {query, filename, case_sensitive})
    }
  }
}

pub fn grep(config: Config) -> Result<(), Box<dyn Error>> {
  let content = fs::read_to_string(&config.filename)?;

  for line in search(&config, &content) {
    println!("{}", line);
  }

  Ok(())
}

pub fn search<'a>(config: &Config, content: &'a str) -> Vec<&'a str> {
  content.lines().filter(|line| line.contains(&config.query) ||
    (!config.case_sensitive &&
      line.to_lowercase().contains(&config.query.to_lowercase()))
  ).collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn one_result() {
    let config = Config{ query: String::from("src"), filename: String::from(""), case_sensitive: false};
    let c = "
    |ac
src is not here
    |dd
    |32";

    assert_eq!(vec!["src is not here"], search(&config, c));
  }
}
