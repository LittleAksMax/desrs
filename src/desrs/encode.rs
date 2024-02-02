
static IP: [u8; 64] = [
    58, 50, 42, 34, 26, 18, 10,  2,
    60, 52, 44, 36, 28, 20, 12,  4,
    62, 54, 46, 38, 30, 22, 14,  6,
    64, 56, 48, 40, 32, 24, 16,  8,
    57, 49, 41, 33, 25, 17,  9,  1,
    59, 51, 43, 35, 27, 19, 11,  3,
    61, 53, 45, 37, 29, 21, 13,  5,
    63, 55, 47, 39, 31, 23, 15,  7,
];

static IP_INV: [u8; 64] = [
    40,  8, 48, 16, 56, 24, 64, 32, 
    39,  7, 47, 15, 55, 23, 63, 31, 
    38,  6, 46, 14, 54, 22, 62, 30, 
    37,  5, 45, 13, 53, 21, 61, 29, 
    36,  4, 44, 12, 52, 20, 60, 28, 
    35,  3, 43, 11, 51, 19, 59, 27, 
    34,  2, 42, 10, 50, 18, 58, 26, 
    33,  1, 41,  9, 49, 17, 57, 25, 
];

static E: [u8; 48] = [
    32,  1,  2,  3,  4,  5, 
     4,  5,  6,  7,  8,  9, 
     8,  9, 10, 11, 12, 13, 
    12, 13, 14, 15, 16, 17, 
    16, 17, 18, 19, 20, 21, 
    20, 21, 22, 23, 24, 25, 
    24, 25, 26, 27, 28, 29,
    28, 29, 30, 31, 32,  1, 
];

static S1: [u8; 64] = [
    14,  4, 13,  1,  2, 15, 11,  8,  3, 10,  6, 12,  5,  9,  0,  7,
     0, 15,  7,  4, 14,  2, 13,  1, 10,  6, 12, 11,  9,  5,  3,  8,
     4,  1, 14,  8, 13,  6,  2, 11, 15, 12,  9,  7,  3, 10,  5,  0,
    15, 12,  8,  2,  4,  9,  1,  7,  5, 11,  3, 14, 10,  0,  6, 13,
];

static S2: [u8; 64] = [
    15,  1,  8, 14,  6, 11,  3,  4,  9,  7,  2, 13, 12,  0,  5, 10,
     3, 13,  4,  7, 15,  2,  8, 14, 12,  0,  1, 10,  6,  9, 11,  5,
     0, 14,  7, 11, 10,  4, 13,  1,  5,  8, 12,  6,  9,  3,  2, 15,
    13,  8, 10,  1,  3, 15,  4,  2, 11,  6,  7, 12,  0,  5, 14,  9
];

static S3: [u8; 64] = [
    10,  0,  9, 14,  6,  3, 15,  5,  1, 13, 12,  7, 11,  4,  2,  8,
    13,  7,  0,  9,  3,  4,  6, 10,  2,  8,  5, 14, 12, 11, 15,  1,
    13,  6,  4,  9,  8, 15,  3,  0, 11,  1,  2, 12,  5, 10, 14,  7,
     1, 10, 13,  0,  6,  9,  8,  7,  4, 15, 14,  3, 11,  5,  2, 12,
];

static S4: [u8; 64] = [
    7, 13, 14,  3,  0,  6,  9, 10,  1,  2,  8,  5, 11, 12,  4, 15,
    13,  8, 11,  5,  6, 15,  0,  3,  4,  7,  2, 12,  1, 10, 14,  9,
    10,  6,  9,  0, 12, 11,  7, 13, 15,  1,  3, 14,  5,  2,  8,  4,
     3, 15,  0,  6, 10,  1, 13,  8,  9,  4,  5, 11, 12,  7,  2, 14,
];

static S5: [u8; 64] = [
     2, 12,  4,  1,  7, 10, 11,  6,  8,  5,  3, 15, 13,  0, 14,  9,
    14, 11,  2, 12,  4,  7, 13,  1,  5,  0, 15, 10,  3,  9,  8,  6,
     4,  2,  1, 11, 10, 13,  7,  8, 15,  9, 12,  5,  6,  3,  0, 14,
    11,  8, 12,  7,  1, 14,  2, 13,  6, 15,  0,  9, 10,  4,  5,  3,
];

static S6: [u8; 64] = [
    12,  1, 10, 15,  9,  2,  6,  8,  0, 13,  3,  4, 14,  7,  5, 11,
    10, 15,  4,  2,  7, 12,  9,  5,  6,  1, 13, 14,  0, 11,  3,  8,
     9, 14, 15,  5,  2,  8, 12,  3,  7,  0,  4, 10,  1, 13, 11,  6,
     4,  3,  2, 12,  9,  5, 15, 10, 11, 14,  1,  7,  6,  0,  8, 13,

];

static S7: [u8; 64] = [
     4, 11,  2, 14, 15,  0,  8, 13,  3, 12,  9,  7,  5, 10,  6,  1,  
    13,  0, 11,  7,  4,  9,  1, 10, 14,  3,  5, 12,  2, 15,  8,  6,  
     1,  4, 11, 13, 12,  3,  7, 14, 10, 15,  6,  8,  0,  5,  9,  2,  
     6, 11, 13,  8,  1,  4, 10,  7,  9,  5,  0, 15, 14,  2,  3, 12,  
];

static S8: [u8; 64] = [
    13,  2,  8,  4,  6, 15, 11,  1, 10,  9,  3, 14,  5,  0, 12,  7,  
     1, 15, 13,  8, 10,  3,  7,  4, 12,  5,  6, 11,  0, 14,  9,  2,  
     7, 11,  4,  1,  9, 12, 14,  2,  0,  6, 10, 13, 15,  3,  5,  8, 
     2,  1, 14,  7,  4, 10,  8, 13, 15, 12,  9,  0,  3,  5,  6, 11,  
];

static P: [u8; 32] = [
    16,  7, 20, 21, 
    29, 12, 28, 17, 
     1, 15, 23, 26, 
     5, 18, 31, 10, 
     2,  8, 24, 14, 
    32, 27,  3,  9, 
    19, 13, 30,  6,
    22, 11,  4, 25, 
];

fn ip(m: u64) -> u64 {
    let mut m_prime: u64 = 0;
    for i in 0..64 {
        m_prime <<= 1;
        m_prime |= (m >> (64 - IP[i])) & 1;
    }
    m_prime
}

fn ip_inv(m_prime: u64) -> u64 {
    let mut m: u64 = 0;
    for i in 0..64 {
        m <<= 1;
        m |= (m_prime >> (64 - IP_INV[i])) & 1;
    }
    m
}

fn expand(r: u32) -> u64 {
    let mut rexp: u64 = 0;
    for i in 0..48 {
        rexp <<= 1;
        rexp |= (r as u64 >> (32 - E[i])) & 1;
    }
    assert!(rexp >> 48 == 0);
    rexp
}

fn sbox(bn: u8, s: &'static [u8; 64]) -> u8 {
    assert!(bn >> 6 == 0);
    // first and last bits 
    let row: u8 = ((bn >> 4) & 2) | (bn & 1);
    // middle 4 bits
    let col: u8 = (bn >> 1) & 0xF;

    (*s)[row as usize * 16 + col as usize]
}

fn combine_and_expand_r(r: u32, k: u64) -> u64 {
    expand(r) ^ k
}

fn permute(f: u32) -> u32 {
    let mut p = 0;
    for i in 0..32 {
        p <<= 1;
        p |= f >> (32 - P[i]) & 1;
    }
    p
}

fn f(r: u32, k: u64) -> u32 {
    assert!(k >> 48 == 0);

    let rnext = combine_and_expand_r(r, k);

    // split rnext into its bytes and go through S-Boxes
    let b1 = sbox((rnext >> 42) as u8 & 0x3F, &S1);
    let b2 = sbox((rnext >> 36) as u8 & 0x3F, &S2);
    let b3 = sbox((rnext >> 30) as u8 & 0x3F, &S3);
    let b4 = sbox((rnext >> 24) as u8 & 0x3F, &S4);
    let b5 = sbox((rnext >> 18) as u8 & 0x3F, &S5);
    let b6 = sbox((rnext >> 12) as u8 & 0x3F, &S6);
    let b7: u8 = sbox((rnext >>  6) as u8 & 0x3F, &S7);
    let b8 = sbox( rnext        as u8 & 0x3F, &S8);

    let combined: u32 = ((b1 as u32) << 28) | ((b2 as u32) << 24) | ((b3 as u32) << 20) | ((b4 as u32) << 16) | ((b5 as u32) << 12) | ((b6 as u32) << 8) | ((b7 as u32) << 4) | (b8 as u32);
    
    permute(combined)
}

pub fn encryption_rounds(m: u64, keys: &[u64; 16]) -> u64 {
    // split 64 bit block into left and right halves
    let l0: u32 = (m >> 32) as u32;
    let r0: u32 = (m & 0xFFFFFFFF) as u32;
    
    let mut l: u32 = l0;
    let mut r: u32 = r0;

    for i in 0..16 {
        // L(n+1) = Rn
        // R(n+1) = Ln XOR f(Rn, K(n+1))

        l = r;
        r = l ^ f(r, keys[i]);
    }

    ip_inv((l as u64) << 32 | (r as u64))
}


pub fn decryption_rounds(m: u64, keys: &[u64; 16]) -> u64 {
    // split 64 bit block into left and right halves
    let l0: u32 = (m >> 32) as u32;
    let r0: u32 = (m & 0xFFFFFFFF) as u32;
    
    let mut l: u32 = l0;
    let mut r: u32 = r0;

    for i in (0..16).rev() {
        // L(n+1) = Rn
        // R(n+1) = Ln XOR f(Rn, K(n+1))

        l = r;
        r = l ^ f(r, keys[i]);
    }

    ip((l as u64) << 32 | (r as u64))
}


#[cfg(test)]
mod tests {
    use super::{ip, expand, combine_and_expand_r, sbox, permute, f, S1};

    static M: u64 = 0b0000000100100011010001010110011110001001101010111100110111101111;
    static M_PRIME: u64 = 0b1100110000000000110011001111111111110000101010101111000010101010;

    static K1: u64 = 0b000110110000001011101111111111000111000001110010;
    static R0: u32 = 0b11110000101010101111000010101010;
    static K1_PLUS_E_OF_R0: u64 = 0b011000010001011110111010100001100110010100100111;

    static E_OF_R0: u64 = 0b011110100001010101010101011110100001010101010101;

    static B1: u8 = 0b011011;
    static S1_OF_B1: u8 = 0b0101;

    static COMBINED_SB: u32 = 0b01011100100000101011010110010111;
    static F: u32 = 0b00100011010010101010100110111011;

    #[test]
    fn expand_works_correctly() {
        assert_eq!(E_OF_R0, expand(R0));
    }

    #[test]
    fn full_key_combination_expands_correctly() {
        assert_eq!(K1_PLUS_E_OF_R0, combine_and_expand_r(R0, K1));
    }

    #[test]
    fn ip_permutes_correctly() {
        assert_eq!(ip(M), M_PRIME);
    }

    #[test]
    fn sbox_works_correctly() {
        assert_eq!(S1_OF_B1, sbox(B1, &S1));
    }

    #[test]
    fn final_permute_works_correctly() {
        assert_eq!(F, permute(COMBINED_SB))
    }

    #[test]
    fn feistel_function_works_correctly() {
        assert_eq!(F, f(R0, K1));
    }
}