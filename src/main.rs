use std::{env, process };

use minigrep::{Config, run};

fn main() {
    let config: Vec<String> = env::args().collect();

    let config = Config::build(&config).unwrap_or_else(|err| {
        eprintln!("Problem parsing argument: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    };
}
