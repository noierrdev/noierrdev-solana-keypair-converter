use std::fs;
use std::path::Path;
use serde_json::from_str;
use anyhow::{Context, Result};
use tokio;

#[tokio::main]
async fn main() {
    let dir = "./"; // path to your directory
    for entry in fs::read_dir(dir) {
    }
}