use std::env;
use std::fs;
use text_colorizer::*;

fn print_usage() {
    eprintln!("{} change occurences of a string in a file", "quickreplace".green());
    eprintln!("Usage: quickreplace <search> <replace> <file> <output>");
}

fn parse_args() -> Args {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 4 {
        print_usage();
        eprintln!("{} missing arguments", "Error:".red().bold());
        std::process::exit(1);
    }

    Args {
        search: args[0].clone(),
        replace: args[1].clone(),
        file: args[2].clone(),
        output: args[3].clone(),
    }
}

#[derive(Debug)]
struct Args {
    search: String,
    replace: String,
    file: String,
    output: String,
}

fn main() {
    let args = parse_args();

    let data = match fs::read_to_string(&args.file) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("{} {}", "Error:".red().bold(), err);
            std::process::exit(1);
        }
    };

    match fs::write(&args.output, data.replace(&args.search, &args.replace)) {
        Ok(_) => (),
        Err(err) => {
            eprintln!("{} {}", "Error:".red().bold(), err);
            std::process::exit(1);
        }
    }
}