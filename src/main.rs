use std::env;
use std::process;

use::rfind::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing the arguments: {err}");
        process::exit(1);
    });

    let _ = rfind::run(config);
}
