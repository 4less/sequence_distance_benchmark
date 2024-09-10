

 
pub mod local {
    use std::iter::zip;

    pub fn hamming_fast(query: &[u8], reference: &[u8]) -> i32 {
        if query.len() >= 100 {
            hamming(&query, &reference)
        } else {
            triple_accel::hamming(&query, &reference) as i32
        }
    }

    pub fn hamming_32(s1: &[u8], s2: &[u8]) -> i32 {
        assert!(s1.len() <= 32);
        zip(s1, s2).fold(0, |acc, (a,b)| acc + (a != b) as i32)
    }

    pub fn hamming_64(s1: &[u8], s2: &[u8]) -> i32 {
        assert!(s1.len() <= 64);
        zip(s1, s2).fold(0, |acc, (a,b)| acc + (a != b) as i32)
    }

    pub fn hamming_128(s1: &[u8], s2: &[u8]) -> i32 {
        assert!(s1.len() <= 128);
        zip(s1, s2).fold(0, |acc, (a,b)| acc + (a != b) as i32)
    }

    pub fn hamming(s1: &[u8], s2: &[u8]) -> i32 {
        zip(s1, s2).fold(0, |acc, (a,b)| acc + (a != b) as i32)
    }

    #[cfg(test)]
    mod tests {
        // Note this useful idiom: importing names from outer (for mod tests) scope.
        use super::*;

        #[test]
        fn test_hamming() {
            assert_eq!(hamming(b"ACGT", b"ACGT"), 0);
            assert_eq!(hamming(b"ACGT", b"ACCT"), 1);
            assert_eq!(hamming(b"ACGT", b"XXXX"), 4);
        }
    }
}