use sha2::{Digest, Sha512};
use std::fs::File;
use std::io;

pub fn file(file_path: &str) -> String {
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
    // convert to a hex string
    format!("{:x}", hash)
}

// u8_data creates a sha512 hash from u8 data passed in.
pub fn u8_data(data_to_hash: &[u8]) -> String {
    let hash = sha2::Sha512::digest(data_to_hash);
    // println!("{:x}", hash);  // used to see output for testing
    format!("{:x}", hash)
}

// String creates a sha512 hash from a string passed in
pub fn string(string_to_hash: &str) -> String {
    let mut sha = Sha512::new();
    sha.input(string_to_hash);
    let hash = sha.result();
    format!("{:x}", hash)
}

// Test with same input and get the same hash string
#[test]
fn test_u8_data() {
    let result = u8_data(b"Hello World! 1234567**");
    println!("{}", result);
    assert_eq!(result,"3c8723969d931f4a1ea70427f2ec7ceb1e921dab3bf5ec72b68ff84197a6551500765095496630ace00908707b6c31df6bb55e9d3b80afee992c221dbca62342");
}

#[test]
fn test_string() {
    let result = string("Hello World! 1234567**");
    assert_eq!(result,"3c8723969d931f4a1ea70427f2ec7ceb1e921dab3bf5ec72b68ff84197a6551500765095496630ace00908707b6c31df6bb55e9d3b80afee992c221dbca62342");
}

#[test]
fn test_file_hello() {
    let result = file("../test-files/hello.txt");
    assert_eq!(result,"3c8723969d931f4a1ea70427f2ec7ceb1e921dab3bf5ec72b68ff84197a6551500765095496630ace00908707b6c31df6bb55e9d3b80afee992c221dbca62342");
}

#[test]
fn test_file() {
    let result = file("../test-files/file.txt");
    assert_eq!(result,"8f24f25be6e3b7cd933aea10cc9599ad77a8d3e9431d3ab08d7009a37cdf5d6d1ba4eaa243745d9ebeff78006b8f3385a365807391f79d8fd9cb12a50d1cdcec");
}
