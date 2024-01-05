mod keygen;
mod encode;

macro_rules! u64_from_bytes {
    ($bytes:expr) => {
        (($bytes[0] as u64) << 56) |
        (($bytes[1] as u64) << 48) |
        (($bytes[2] as u64) << 40) |
        (($bytes[3] as u64) << 32) |
        (($bytes[4] as u64) << 24) |
        (($bytes[5] as u64) << 16) |
        (($bytes[6] as u64) << 8) |
        ($bytes[7] as u64)
    };
}

pub fn encrypt(data: &[u8], key: u64) -> Vec<u64> {
    let keys = keygen::create_subkeys(key);
    let mut data_padded: Vec<u8> = Vec::from(data);
    let extra_bytes = data_padded.len() % 8;

    for _ in 0..(8 - extra_bytes) {
        data_padded.push(0x00);
    }

    let mut encrypted: Vec<u64> = Vec::new();

    for i in 0..(data_padded.len() / 8) {
        encrypted.push(encode::encryption_rounds(u64_from_bytes!(data_padded[(i*8)..]), &keys));
    }

    return encrypted;
}

pub fn decrypt(data: &[u8], key: u64) -> String {
    println!("[DECRYPTING]");
    dbg!(data);
    dbg!(key);
    String::from("")
}
