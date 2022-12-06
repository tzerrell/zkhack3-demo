#![no_main]

use risc0_zkvm_guest::{env, sha};

risc0_zkvm_guest::entry!(main);

pub fn main() {
    let pw: String = env::read();

    let mut is_ok = false;
    for ch in pw.chars() {
        if ch.is_ascii_punctuation() {
            is_ok = true;
        }
    }
    if !is_ok {
        panic!();
    }

    let digest = sha::digest_u8_slice(pw.as_bytes());

    env::commit(digest);
}
