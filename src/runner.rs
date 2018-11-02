use grep::Config;
use grep::grep;
use std::env;
use std::process;

pub fn run() {
  let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problems: {}", err);
        process::exit(1);
    });

    println!("Config: {}, {}", config.query, config.filename);

    if let Err(e) = grep(config) {
        eprintln!("Application Failed: {}", e);
        process::exit(1);
    }
}
