use std::{env, process};

use minigrep::{run, Config};

fn main() {
    let mut args = env::args();
    args.next();
    let config = Config::build(args).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {err}");
        process::exit(1);
    });
    if let Err(err) = run(config) {
        eprintln!("Application error: {err}");
        process::exit(1);
    }
}
