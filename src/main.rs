use std::fs;
use std::path::Path;
use serde_json::from_str;

#[tokio::main]
async fn main() {
    let dir = "./"; // path to your directory
    while let Ok(entry) in fs::read_dir(dir) {
        let entry = entry;
        // println!("{:?}", entry);
        // let path = entry.path();

        // if path.extension().and_then(|s| s.to_str()) == Some("json") {
        //     // Read file
        //     let contents = fs::read_to_string(&path);
        //     let secret_bytes: Vec<u8> = from_str(&contents).expect("valid JSON array");
        //     let secret_arr: [u8; 64] = secret_bytes.try_into().expect("64-byte secret");

        //     // Encode as base58 string
        //     let encoded = bs58::encode(secret_arr).into_string();

        //     // Save string representation to file
        //     let out_path = path.with_extension("txt");
        //     fs::write(&out_path, &encoded);

        //     // Remove the original .json file
        //     // fs::remove_file(&path)?;

        //     println!("Converted and deleted {:?}", path);
        // }
    }
}