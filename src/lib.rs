const HEADERS: [&str; 46] = include!("./headers.rs");

pub fn is_header_const(line: &str) -> bool {
    HEADERS.iter().any(|h| &line == h)
}

use xxhash_rust::xxh3::xxh3_64;

pub fn is_header_hash(line: &str) -> bool {
    let hash = xxh3_64(line.as_bytes());

    include!(concat!(env!("OUT_DIR"), "/hashes.rs"))
}
