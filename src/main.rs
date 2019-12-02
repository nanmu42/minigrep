use std::env;
use std::process;

use minigrep;

const CASE_INSENSITIVE_KEY: &str = "CASE_INSENSITIVE";

fn main() {
    let args: Vec<String> = env::args().collect();
    let case_insensitive = env::var(CASE_INSENSITIVE_KEY).is_err();
    let config = minigrep::Config::new(&args, !case_insensitive).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(err) = minigrep::run(&config) {
        eprintln!("Problem running match: {}", err);
        process::exit(1);
    };
}
