use std::path::PathBuf;
use xxhash_rust::xxh3::xxh3_64;

const HEADERS: [&str; 46] = include!("./src/headers.rs");

fn main() {
    let hashes = HEADERS
        .iter()
        .map(|s| {
            let hash = xxh3_64(s.as_bytes());

            format!("    {hash} => true,\n")
        })
        .collect::<String>();
    let src = format!("match hash {{\n{hashes}    _ => false,\n}}\n");

    let path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    std::fs::write(path.join("hashes.rs"), src).unwrap();
}
