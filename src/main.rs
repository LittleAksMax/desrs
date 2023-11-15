use std::{env, process};
use desrs::desrs::{encrypt, decrypt};

macro_rules! invalid_args {
    ($extra:expr) => {
        eprintln!("{}", $extra);
        eprintln!("USAGE: desrs [ed] <data> <key>");
        process::exit(1);           
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        invalid_args!("You need 4 arguments.");
    }

    if !args[1].is_ascii() || !args[2].is_ascii() || !args[3].is_ascii() {
        invalid_args!("All arguments must be ASCII.");
    }

    if args[1].len() != 1 || (!args[1].ends_with('e') && !args[1].ends_with('d')) {
        invalid_args!("[ed] argument must be either simply 'e' or 'd'");
    }

    // extract what should be the only character in the first argument
    let mode = args[1].chars().next().unwrap();

    let data = args[2].as_bytes();

    let key_bytes = args[3].as_bytes();

    if key_bytes.len() != 8 {
        invalid_args!("Key must be 64 bits (8 ASCII characters)");
    }

    if mode == 'e' {
        //encrypt(data, key);
    } else if mode == 'd' {
        //decrypt(data, key);
    } else {
        assert!(false);
    }
}