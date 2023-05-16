extern crate hex;
extern crate sha2; 
extern crate rand;


use sha2::{Digest, Sha256};
use rand::Rng;

fn main() {
    let id_hex = "ED00AF5F774E4135E7746419FEB65DE8AE17D6950C95CEC3891070FBB5B03C77";
    let id_bytes = hex::decode(id_hex).expect("Decoding failed");

    let mut rng = rand::thread_rng();
    let mut x_bytes: Vec<u8>;

    loop {
        x_bytes = (0..32).map(|_| rng.gen()).collect();
        let combined = [&x_bytes[..], &id_bytes[..]].concat();
        let hash = Sha256::digest(&combined);

        if hash.iter().any(|&byte| byte == 0x1D) {
            break;
        }
    }   

    println!("Found x: {}", hex::encode(&x_bytes));
}