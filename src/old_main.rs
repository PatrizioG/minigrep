use std::env;
use std::process;

fn main()
{
    let args: Vec<String> = env::args().collect();

    let config = minigrep::Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[0];
    let filename = &args[1];

    (query, filename)
}


