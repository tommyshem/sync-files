use sha2::{Digest, Sha512};
use std::fs::File;
use std::io;

fn main() {
    let stringToHash = "Hello World!\n";
    hashString(stringToHash);
    hashFile("../test-files/file.txt")
}

fn hashFile(filePath: &str) {
    let mut f = File::open(&filePath);
    let mut f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let mut sha = Sha512::new();
    let n = io::copy(&mut f, &mut sha).expect("error");
    // Note that calling `result()` consumes hasher
    let hash = sha.result();
    println!("Path: {}", filePath);
    println!("Bytes processed: {}", n);
    println!("Hash value: {:x}", hash);
    //return hash;
}

fn hashString(datatohash: &str) {
    //-> [u8; 64] {
    let mut sha = Sha512::new();
    sha.input(datatohash);
    // Note that calling `result()` consumes hasher
    let hash = sha.result();
    println!("{:x}", hash);
    //let res: [u8; 64] = sha.result().into()
    //return hash;
}
