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

#[cfg(test)]
mod tests {
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn digest() {
        assert_eq!(super::digest("foobar"), "c3ab8ff13720e8ad9047dd39466b3c8974e592c2fa383d4a3960714caef0c4f2".to_string());
        assert_eq!(super::digest(""), "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".to_string());
    }

    #[test]
    fn epoch_now() {
        let first = super::epoch_now();
        sleep(Duration::new(1, 0));
        let second = super::epoch_now();
        assert!(second >= first + 1);
    }

    #[test]
    fn gen_random_hash() {
        let first = super::gen_random_hash();
        let second = super::gen_random_hash();
        assert!(first != second);
        assert_eq!(first.len(), 64);
        assert_eq!(second.len(), 64);
    }
}