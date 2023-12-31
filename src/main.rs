use std::{env, process};
use desrs::desrs::{encrypt, decrypt};

macro_rules! invalid_args {
    ($extra:expr) => {
        eprintln!("{}", $extra);
        eprintln!("USAGE: desrs [ed] <data> <key>");
        process::exit(1);           
    };
}

macro_rules! u64_from_bytes {
    ($bytes:expr) => {
        (($bytes[0] as u64) << 56) |
        (($bytes[1] as u64) << 48) |
        (($bytes[2] as u64) << 40) |
        (($bytes[3] as u64) << 32) |
        (($bytes[4] as u64) << 24) |
        (($bytes[5] as u64) << 16) |
        (($bytes[6] as u64) << 8) |
        ($bytes[7] as u64)
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

    let key = u64_from_bytes!(key_bytes);

    if mode == 'e' {
        dbg!("{}", encrypt(data, key));
    } else if mode == 'd' {
        dbg!("{}", decrypt(data, key));
    } else {
        assert!(false);
        std::process::exit(1);
    }
}