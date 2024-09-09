use criterion::black_box;
use libwfa::{affine_wavefront::AffineWavefronts, mm_allocator::MMAllocator, penalties::AffinePenalties};
use libwfa::bindings::BUFFER_SIZE_8M;

pub fn wfa() {
    let alloc = MMAllocator::new(BUFFER_SIZE_8M as u64);

    let pattern = String::from("TCTTTACTCGCGCGTTGGAGAAATACAATAGT");
    let text = String::from("TCTATACTGCGCGTTTGGAGAAATAAAATAGT");

    let mut penalties = AffinePenalties {
        match_: 0,
        mismatch: 4,
        gap_opening: 6,
        gap_extension: 2,
    };

    let pat_len = pattern.as_bytes().len();
    let text_len = text.as_bytes().len();

    let mut wavefronts = AffineWavefronts::new_complete(
        pat_len,
        text_len,
        &mut penalties,
        &alloc,
    );

    wavefronts
        .align(pattern.as_bytes(), text.as_bytes())
        .unwrap();

    let score = wavefronts.edit_cigar_score(&mut penalties);

    println!("score: {}", score);
    wavefronts.print_cigar(pattern.as_bytes(), text.as_bytes());

    // The cigar can also be extracted as a byte vector
    let cigar = wavefronts.cigar_bytes_raw();
    let cg_str = std::str::from_utf8(&cigar).unwrap();
    println!("cigar: {}", cg_str);

    // Or as a prettier byte vector

    let cigar = wavefronts.cigar_bytes();
    let cg_str = std::str::from_utf8(&cigar).unwrap();
    println!("cigar: {}", cg_str);
}

pub fn libwfa(query: &[u8], reference: &[u8], alloc: &MMAllocator, penalties: &mut AffinePenalties) -> isize {
    let pat_len = query.len();
    let text_len = reference.len();
    let mut wavefronts = AffineWavefronts::new_complete(
        pat_len,
        text_len,
        penalties,
        &alloc,
    );
    wavefronts
        .align(query, reference)
        .unwrap();
    black_box(wavefronts.edit_cigar_score(penalties))
}