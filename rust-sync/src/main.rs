// let hex = hasher.result_str();

//assert_eq!(hex,
  //         concat!("b94d27b9934d3e08a52e52d7da7dabfa",
    //               "c484efe37a5380ee9088f7ace2efcde9"));
      //             }


//extern crate crypto;

//use crypto::digest::Digest;
//use crypto::sha2::Sha256;
use sha2::{Sha256, Sha512, Digest};
fn main() {
    let input = "Hello world!";
    let mut sha = Sha256::new();
    sha.input_str(input);
    println!("{}", sha.result_str());
}