use std::env;
use std::process;

use command_line::Config;

fn main() {


    let config = Config::build(env::args()).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // run(config);
    if let Err(e) = command_line::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}