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

pub fn encrypt(data: &[u8], key: u64) -> Vec<u8> {
    let keys = keygen::create_subkeys(key);
    let mut data_padded: Vec<u8> = Vec::from(data);
    let extra_bytes = data_padded.len() % 8;

    if extra_bytes != 0 {
        for _ in 0..(8 - extra_bytes) {
            data_padded.push(0x00);
        }
    }

    let mut encrypted: Vec<u8> = Vec::new();

    for i in 0..(data_padded.len() / 8) {
        let u64_enc = encode::encryption_rounds(u64_from_bytes!(data_padded[(i*8)..]), &keys);
        encrypted.push((u64_enc >> 56) as u8);
        encrypted.push(((u64_enc >> 48) as u8) & 0xFF);
        encrypted.push(((u64_enc >> 40) as u8) & 0xFF);
        encrypted.push(((u64_enc >> 32) as u8) & 0xFF);
        encrypted.push(((u64_enc >> 24) as u8) & 0xFF);
        encrypted.push(((u64_enc >> 16) as u8) & 0xFF);
        encrypted.push(((u64_enc >> 8) as u8) & 0xFF);
        encrypted.push((u64_enc as u8) & 0xFF);
    }

    encrypted
}

pub fn decrypt(data: &[u8], key: u64) -> Vec<u8> {
    println!("[DECRYPTING]");
    dbg!(data);
    dbg!(key);
    vec![]
}
