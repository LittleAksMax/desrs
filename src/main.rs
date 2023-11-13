use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("USAGE: desrs [ed] <string>");
        process::exit(1);
    }

    let mode = &args[1];
    let data = &args[2];

    if !(*mode).is_ascii() || !(*data).is_ascii() {
        eprintln!("USAGE: desrs [ed] <string>");
        process::exit(1);
    }

    if (*mode).len() != 1 || (!(*mode).ends_with('e') && !(*mode).ends_with('d')) {
        eprintln!("USAGE: desrs [ed] <string>");
        process::exit(1);
    }
}