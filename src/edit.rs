

 
pub mod local {
    use std::iter::zip;

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