use std::fs;
use std::path::Path;
use serde_json::{Value, from_str};
use solana_sdk::{ signature::{Keypair,Signature,Signer}};


#[tokio::main]
async fn main() {
    let dir = "./"; // path to your directory
    let mut dir_entries=fs::read_dir(dir).unwrap();
    for entry in  dir_entries{
        let entry = entry.unwrap();
        
        let path = entry.path();
        

        if path.is_dir() ==false && path.extension().and_then(|s| s.to_str()) == Some("json") {
            // Read file
            println!("{:?}", path);

            let contents = match fs::read_to_string(&path) {
                Ok(c) => c,
                Err(_) => {
                    println!("Could not read {:?}", path);
                    continue;
                }
            };
    

            let secret_bytes: Vec<u8> = match serde_json::from_str(&contents) {
                Ok(b) => b,
                Err(_) => {
                    println!("Invalid JSON in {:?}", path);
                    continue;
                }
            };
            let secret_arr: [u8; 64] = match secret_bytes.try_into() {
                Ok(arr) => arr,
                Err(_) => {
                    println!("Not 64 bytes in {:?}", path);
                    continue;
                }
            };

            let keypair = match Keypair::from_bytes(&secret_arr) {
                Ok(kp) => kp,
                Err(err) => {
                    println!("Error creating keypair from {:?}: {}", path, err);
                    continue;
                }
            };
            // let secret_bytes: Vec<u8> = from_str(&contents).expect("valid JSON array");
            // let secret_arr: [u8; 64] = secret_bytes.try_into().expect("64-byte secret");

            // // Encode as base58 string
            let private_key = bs58::encode(keypair.secret_bytes()).into_string();
            let public_key = keypair.try_pubkey().to_string();

            println!("{:?}",public_key);
            println!("{:?}", private_key);

            // // Save string representation to file
            // let out_path = path.with_extension("txt");
            // fs::write(&out_path, &encoded);

            // // Remove the original .json file
            // // fs::remove_file(&path)?;

            println!("Converted and deleted {:?}", path);
        }
    }
}

fn is_valid_json(s: &str) -> bool {
    serde_json::from_str::<Value>(s).is_ok()
}