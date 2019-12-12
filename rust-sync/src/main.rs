mod hash;

use std::env;
use std::iter::Iterator;
use std::process;

pub fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: {} <source> <target>", args[0]);
        process::exit(1);
    }
}
