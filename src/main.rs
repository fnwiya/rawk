extern crate regex;
extern crate getopts;

use std::env;
use getopts::Options;

fn main() {
    let args: Vec<String> = env::args().collect();
    parse(args);
}

fn parse(args: Vec<String>) {
    let program = args[0].clone();
    let mut opts = Options::new();

    opts.optflag("h", "help", "show help");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    let with_option_h = matches.opt_present("h");
    if with_option_h {
        print_usage(&program, opts);
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}
