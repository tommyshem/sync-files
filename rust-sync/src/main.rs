use sha2::{Digest, Sha512};
use std::fs::File;
use std::io;

fn main() {
    let string_to_hash = "Hello World!\n";
    hash_string(string_to_hash);
    hash_file("../test-files/file.txt")
}

fn hash_file(file_path: &str) {
    let mut f = match File::open(&file_path) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let mut sha = Sha512::new();
    let n = io::copy(&mut f, &mut sha).expect("error");
    // Note that calling `result()` consumes sha
    let hash = sha.result();
    println!("Path: {}", file_path);
    println!("Bytes processed: {}", n);
    println!("Hash value: {:x}", hash);
    //return hash;
}

fn hash_string(data_to_hash: &str) {
    //-> [u8; 64] {
    let mut sha = Sha512::new();
    sha.input(data_to_hash);
    // Note that calling `result()` consumes sha
    let hash = sha.result();
    println!("{:x}", hash);
    //let res: [u8; 64] = sha.result().into()
    //return hash;
}
