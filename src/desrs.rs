mod keygen;
mod encode;

pub fn encrypt(data: &[u8], key: u64) {
    println!("[ENCRYPTING]");
    dbg!(data);
    dbg!(key);
}

pub fn decrypt(data: &[u8], key: u64) {
    println!("[DECRYPTING]");
    dbg!(data);
    dbg!(key);
}
