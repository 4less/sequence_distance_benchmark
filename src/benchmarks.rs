// extern crate test;


// #[cfg(test)]
// mod tests {
//     use std::{cmp::min, intrinsics::black_box, iter::zip};

//     use crate::{edit::local::hamming, test_data::test::Data};
//     use crate::alignment::libwfa;
//     use libwfa::{affine_wavefront::AffineWavefronts, bindings::BUFFER_SIZE_8M, mm_allocator::MMAllocator, penalties::AffinePenalties};

//     use super::*;
//     use edit_distance::edit_distance;
//     use levenshtein::levenshtein;
//     use lib::{alignment_lib::Penalties, wavefront_alignment::wavefront_align};
//     use rust_hamming_distance::hamming_distance::HammingDistancable;
//     use seqalign::{measures::LevenshteinDamerau, Align};
//     use test::Bencher;

//     #[bench]
//     fn bench_hamming(b: &mut Bencher) {
        
//         let data = Data::generate(0.05, 0.01, 150, 50);
    
//         b.iter(|| data.zip_seq().for_each(|(query,reference)| { 
//             black_box(hamming(query, reference)); 
//         }));
//     }

//     #[bench]
//     fn bench_levensthein(b: &mut Bencher) {
        
//         let data = Data::generate(0.05, 0.01, 150, 50);
    
//         b.iter(|| data.zip_seq().for_each(|(query,reference)| { 
//             black_box(levenshtein(&String::from_utf8(query.to_vec()).unwrap(), &String::from_utf8(reference.to_vec()).unwrap())); 
//         }));
//     }

//     #[bench]
//     fn bench_edit_distance(b: &mut Bencher) {
        
//         let data = Data::generate(0.05, 0.01, 150, 50);
    
//         b.iter(|| data.zip_seq().for_each(|(query,reference)| { 
//             black_box(edit_distance(&String::from_utf8(query.to_vec()).unwrap(), &String::from_utf8(reference.to_vec()).unwrap())); 
//         }));
//     }

//     #[bench]
//     fn bench_levensthein_damerau(b: &mut Bencher) {
//         let data = Data::generate(0.05, 0.01, 150, 50);
//         let measure = LevenshteinDamerau::new(1, 1, 1, 1);
//         b.iter(|| data.zip_seq().for_each(|(query,reference)| { 
//             black_box(measure.align(query, reference)); 
//         }));
//     }

//     #[bench]
//     fn bench_external_rust_hamming_distance(b: &mut Bencher) {    
//         let data = Data::generate(0.05, 0.01, 150, 50);
    
//         b.iter(|| data.zip_seq().for_each(|(query,reference)| { 
//             let len = min(query.len(), reference.len());
//             black_box(&query[0..len].hamming_distance(&&reference[0..len]).unwrap()); 
//         }));
//     }

//     #[bench]
//     fn bench_external_triple_accel_hamming_distance(b: &mut Bencher) {    
//         let data = Data::generate(0.05, 0.01, 150, 50);
    
//         b.iter(|| data.zip_seq().for_each(|(query,reference)| { 
//             let len = min(query.len(), reference.len());
//             black_box(triple_accel::hamming(&query[0..len], &reference[0..len])); 
//         }));
//     }

//     #[bench]
//     fn bench_external_triple_accel_edit_distance(b: &mut Bencher) {    
//         let data = Data::generate(0.05, 0.01, 150, 50);
    
//         b.iter(|| data.zip_seq().for_each(|(query,reference)| { black_box(triple_accel::levenshtein(query, reference)); }));
//     }

//     #[bench]
//     fn bench_wfa(b: &mut Bencher) {
//         let pens = Penalties {
//             mismatch_pen: 1,
//             open_pen: 6,
//             extd_pen: 2,
//         };

//         let data = Data::generate(0.05, 0.01, 150, 50);

//         b.iter(|| {
//             for (query, reference) in zip(&data.queries, &data.references) {
//                 let _ = black_box(wavefront_align(black_box(&String::from_utf8((query).to_vec()).unwrap()), black_box(&String::from_utf8((reference).to_vec()).unwrap()), black_box(&pens)));
//             }
//         });
//     }

//     #[bench]
//     fn bench_external_libwfa1(b: &mut Bencher) {
//         let alloc = MMAllocator::new(BUFFER_SIZE_8M as u64);

    
//         let mut penalties = AffinePenalties {
//             match_: 0,
//             mismatch: 4,
//             gap_opening: 6,
//             gap_extension: 2,
//         };
    
    
//         let data = Data::generate(0.00, 0.00, 150, 50);


//         b.iter(|| data.zip_seq().for_each(|(query,reference)| { 
//             let pat_len = query.len();
//             let text_len = reference.len();
//             let mut wavefronts = AffineWavefronts::new_complete(
//                 pat_len,
//                 text_len,
//                 &mut penalties,
//                 &alloc,
//             );
//             wavefronts
//                 .align(query, reference)
//                 .unwrap();
//             black_box(wavefronts.edit_cigar_score(&mut penalties));
//         }));
//     }

//     #[bench]
//     fn bench_external_libwfa2(b: &mut Bencher) {
//         let alloc = MMAllocator::new(BUFFER_SIZE_8M as u64);

    
//         let mut penalties = AffinePenalties {
//             match_: 0,
//             mismatch: 4,
//             gap_opening: 6,
//             gap_extension: 2,
//         };
    
    
//         let data = Data::generate(0.05, 0.01, 150, 50);


//         b.iter(|| data.zip_seq().for_each(|(query,reference)| { 
//             let pat_len = query.len();
//             let text_len = reference.len();
//             let mut wavefronts = AffineWavefronts::new_complete(
//                 pat_len,
//                 text_len,
//                 &mut penalties,
//                 &alloc,
//             );
//             wavefronts
//                 .align(query, reference)
//                 .unwrap();
//             black_box(wavefronts.edit_cigar_score(&mut penalties));
//         }));
//     }


//     #[bench]
//     fn bench_external_libwfa3(b: &mut Bencher) {
//         let alloc = MMAllocator::new(BUFFER_SIZE_8M as u64);

    
//         let mut penalties = AffinePenalties {
//             match_: 0,
//             mismatch: 4,
//             gap_opening: 6,
//             gap_extension: 2,
//         };
    
    
//         let data = Data::generate(0.8, 0.1, 150, 50);


//         b.iter(|| data.zip_seq().for_each(|(query,reference)| { 
//             let pat_len = query.len();
//             let text_len = reference.len();
//             let mut wavefronts = AffineWavefronts::new_complete(
//                 pat_len,
//                 text_len,
//                 &mut penalties,
//                 &alloc,
//             );
//             wavefronts
//                 .align(query, reference)
//                 .unwrap();
//             black_box(wavefronts.edit_cigar_score(&mut penalties));
//         }));
//     }


//     #[bench]
//     fn bench_external_libwfa4(b: &mut Bencher) {
//         let alloc = MMAllocator::new(BUFFER_SIZE_8M as u64);    
        
//         let data = Data::generate(0.8, 0.1, 30, 50);

//         let mut penalties = AffinePenalties {
//             match_: 0,
//             mismatch: 4,
//             gap_opening: 6,
//             gap_extension: 2,
//         };

//         b.iter(|| data.zip_seq().for_each(|(query,reference)| { 
//             black_box(libwfa(query, reference, &alloc, &mut penalties));
//         }));
//     }
// }
