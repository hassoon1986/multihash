use std::io::Read;

extern crate crypto;

use self::crypto::digest::Digest;
use self::crypto::sha2::Sha256;

fn main() -> std::io::Result<()> {
    let mut stdin = std::io::stdin();
    let mut buf: [u8; 1024] = [0; 1024];

    let mut hasher = Sha256::new();
    loop {
        match stdin.read(&mut buf) {
            Ok(0) => break,
            Ok(v) => hasher.input(&buf[0..v]),
            Err(_) => break,
        }
    }

    let hex = hasher.result_str();
    println!("sha256: {}", hex);
    Ok(())
}
