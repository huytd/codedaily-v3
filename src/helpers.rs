extern crate rand;
extern crate crypto;

use self::crypto::digest::Digest;
use self::crypto::sha2::Sha256;
use self::rand::{thread_rng, Rng};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn gen_random_hash() -> String {
    let random_str: String = thread_rng().gen_ascii_chars().take(32).collect();
    digest(random_str.as_str())
}

pub fn digest(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(input);
    hasher.result_str()
}

pub fn epoch_now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}
