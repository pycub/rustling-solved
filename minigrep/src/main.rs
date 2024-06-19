use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("{err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In filepath {}", config.filepath);

    if let Err(err) = minigrep::run(config) {
        println!("Application error: {err}");
        process::exit(1);
    }
}
