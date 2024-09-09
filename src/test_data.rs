pub mod test {
    use std::{collections::HashSet, fmt::Display, iter::{zip, Zip}};

    use rand::Rng;

    #[derive(Debug)]
    pub struct Info {
        mutations: usize,
        insertions: usize,
        deletions: usize,
    }

    impl Info {
        pub fn new(mutations: usize, insertions: usize, deletions: usize) -> Self {
            Self {
                mutations,
                insertions,
                deletions
            }
        }
    }

    impl Default for Info {
        fn default() -> Self {
            Self { mutations: 0, insertions: 0, deletions: 0 }
        }
    }

    impl Display for Info {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Mutations: {}  Insertions: {}  Deletions: {}", self.mutations, self.insertions, self.deletions)
        }
    }

    #[derive(Debug)]
    pub struct Data {
        pub queries: Vec::<Vec<u8>>,
        pub references: Vec::<Vec<u8>>,
        pub info: Vec<Info>,
    }



    impl Default for Data {
        fn default() -> Self {
            Self { 
                queries: vec![
                    "ACTCTATTTTACTCAGTGCAGGGTGAGCCGCCTATGCGGAGTGCAGTTACATAGGGAAAGCGGGGCTCAATTGCTACTCGTATGGGGTGTCACAGACGC".into(),
                    "ACTCTATATTACTCAGTGCAGGGTGAGCCGCCTATGCGGTTACATAGGGAAAGCGGGGCTCAAATGCTACTCGTATGGGATGTCACAAACGC".into(),
                    "ACTCTATTTTACTCAGTGCAGGGTGAGCCGCCTATGCGGAGTGCAGTTACATAGGGAAAGCGGGGCTCAATTGCTACTCGTATGGGGTGTCACAGACGC".into(),
                ], 
                references: vec![
                    "ACTCTATTTTACTCAGTGCAGGGTGAGCCGCCTATGCGGAGTGCAGTTACATAGGGTAAAGCGGGGCTCAATTGCTACTCGTATGGGGTGTCACAGACGC".into(),
                    "ACTCTATTTTACTCAGTGCAGGGTGAGCCGCCTATGCGGAGTGCAGTTACATAGGGTAAAGCGGGGCTCAATTGCTACTCGTATGGGGTGTCACAGACGC".into(),
                    "ACTCTATTTTACTCGTAGCTAGCTGCTGTCGTCGTCGTCGTCTAGCAGTTACATAGGGTAAAGCGGGGCTCAATTGCTACTCGTATGGGGTGTCACAGACGC".into(),
                ],
                info: vec![Info::default(), Info::default(), Info::default()],
            }
        }
    }

    impl Display for Data {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", zip(&self.queries, &self.references).fold(String::default(), |acc, (a,b)| {
                format!("{}Pair\n{}\n{}\n", acc, String::from_utf8(a.to_vec()).unwrap(), String::from_utf8(b.to_vec()).unwrap())
            }))
        }
    }

    impl Data {
        const BASES: [u8; 4] = [b'A', b'C', b'G', b'T'];

        pub fn zip_seq(&self) -> Zip<std::slice::Iter<'_, Vec<u8>>, std::slice::Iter<'_, Vec<u8>>>{
            zip(&self.queries, &self.references)
        }

        pub fn zip(&self) -> Zip<Zip<std::slice::Iter<'_, Vec<u8>>, std::slice::Iter<'_, Vec<u8>>>, std::slice::Iter<'_, Info>> {
            zip(&self.queries, &self.references).zip(&self.info)
        }

        fn generate_x_out_of_n(x: usize, n: usize) -> HashSet<usize> {
            let mut rng = rand::thread_rng(); // Create a random number generator
            let mut set = HashSet::default();

            (0..x).for_each(|_|  {
                let mut num = rng.gen_range(0..n);
                while set.contains(&num) {
                    num = rng.gen_range(0..n);
                }
                set.insert(num);
            });
            return set;
        }

        fn generate_random_base(not_base: Option<u8>) -> u8 {
            let mut rng = rand::thread_rng(); // Create a random number generator
            let mut num = rng.gen_range(0..Self::BASES.len());
            let mut base = Self::BASES[num];
            match not_base {
                Some(not_base) => {
                    while base == not_base {
                        num = rng.gen_range(0..Self::BASES.len());
                        base = Self::BASES[num];
                    };
                    return base;
                },
                None => base,
            }
        }
        

        fn generate_random_sequence(len: usize) -> Vec<u8> {
            (0..len).map(|_| Self::generate_random_base(None)).collect::<Vec<u8>>()
        }

        fn alter_sequence(sequence: &mut Vec<u8>, num_mut: usize, num_indel: usize) -> (usize, usize, usize) {
            let mut_pos = Self::generate_x_out_of_n(num_mut, sequence.len());
            let indel_pos = Self::generate_x_out_of_n(num_indel, sequence.len());

            for pos in &mut_pos {
                let base = Self::generate_random_base(Some(sequence[*pos]));
                sequence[*pos] = base;
            }

            let mut insertions = 0;
            let mut deletions = 0;
            let mut rng = rand::thread_rng(); // Create a random number generator
            for _ in (0..num_indel) {
                let pos = rng.gen_range(0..sequence.len());
                let base = Self::generate_random_base(Some(sequence[pos]));
                match rng.gen_bool(0.5) {
                    true => { insertions += 1; sequence.insert(pos, base); },
                    false => { deletions += 1; sequence.remove(pos); },
                }
            };
            (mut_pos.len(), insertions, deletions)
        }

        pub fn generate(mutation_rate: f64, indel_rate: f64, length: usize, n: usize) -> Self {
            let mut queries = Vec::new();
            let mut references = Vec::new();
            let mut info = Vec::new();

            let num_mut = length as f64 * mutation_rate;
            let num_indel = length as f64 * indel_rate;

            for i in 0..n {
                let query = Self::generate_random_sequence(length);
                let mut reference = query.clone();
                let (m, i, d) = Self::alter_sequence(&mut reference, num_mut as usize, num_indel as usize);
                
                queries.push(query);
                references.push(reference);
                info.push(Info::new(m, i, d));
            }

            Self {
                queries,
                references,
                info,
            }
        }
    }
}