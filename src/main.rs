#![feature(test)]
#![feature(core_intrinsics)]

use std::intrinsics::black_box;

use edit::local::hamming;
use lib::{alignment_lib::Penalties, reference, wavefront_alignment::wavefront_align};
use libwfa::{affine_wavefront::AffineWavefronts, mm_allocator::MMAllocator, penalties::AffinePenalties};
use libwfa::bindings::BUFFER_SIZE_8M;
use seqalign::measures::LevenshteinDamerau;
use seqalign::Align;
use test_data::test::Data;

mod edit;
mod benchmarks;
mod alignment;
pub mod test_data;
pub mod benches;

fn test_wfa() {
    let query = "ACTCTATTTTACTCAGTGCAGGGTGAGCCGCCTATGCGGAGTGCAGTTACATAGGGAAAGCGGGGCTCAATTGCTACTCGTATGGGGTGTCACAGACGC";
    let reference = "ACTCTATTTTACTCAGTGCAGGGTGAGCCGCCTATGCGGAGTGCAGTTACATAGGGTAAAGCGGGGCTCAATTGCTACTCGTATGGGGTGTCACAGACGC";

    let pens = Penalties {
        mismatch_pen: 1,
        open_pen: 2,
        extd_pen: 2,
    };
    
    let wavefront_align = wavefront_align(black_box(query), black_box(reference), black_box(&pens));
    let res = wavefront_align;

    eprintln!("{:?}", res);

    let alloc = MMAllocator::new(BUFFER_SIZE_8M as u64);

    let mut penalties = AffinePenalties {
        match_: 0,
        mismatch: 4,
        gap_opening: 6,
        gap_extension: 2,
    };

    let pat_len = query.len();
    let text_len = reference.len();

    let mut wavefronts = AffineWavefronts::new_complete(
        pat_len,
        text_len,
        &mut penalties,
        &alloc,
    );
    wavefronts
        .align(query.as_bytes(), reference.as_bytes())
        .unwrap();



    let score = wavefronts.edit_cigar_score(&mut penalties);

    eprintln!("score: {}", score);
    wavefronts.print_cigar(query.as_bytes(), reference.as_bytes());
}

fn main() {
    // test_wfa();

    let data = Data::generate(0.05, 0.01, 150, 2);
    println!("{}", data);

    let measure = LevenshteinDamerau::new(1, 1, 1, 1);
    for ((a, b),info) in data.zip() {
        eprintln!("Hamming {} Levenshtein {}.. {}", hamming(a, b), measure.align(a,b).distance(), info);
    }
}
