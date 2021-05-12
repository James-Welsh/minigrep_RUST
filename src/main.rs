use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // Iterate over all arguments and 'collect' them into a vector.
    let args: Vec<String> = env::args().collect(); // will panic if not Unicode

    let config: Config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
