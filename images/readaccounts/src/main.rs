use risc0_zkvm::{guest::{env, sha::Impl},sha::{Digest, Sha256}};

fn main() {
    let mut account_bytes_1: Vec<u8> = Vec::new();
    env::read_slice(&mut account_bytes_1);
    let mut account_bytes_2: Vec<u8> = Vec::new();
    env::read_slice(&mut account_bytes_2);
    let mut account_bytes_3: Vec<u8> = Vec::new();
    env::read_slice(&mut account_bytes_3);
    let test_len = account_bytes_1.len() + account_bytes_2.len() + account_bytes_3.len();
    let digest = Impl::hash_bytes(
        &[
            account_bytes_1,
            account_bytes_2,
            account_bytes_3
        ].concat(),
    );
    env::commit_slice(&digest.as_bytes());
    env::commit_slice(&test_len.to_le_bytes());
}
