
use std::cmp::min;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use lib::{alignment_lib::Penalties, wavefront_alignment::wavefront_align};
use libwfa::{bindings::BUFFER_SIZE_8M, mm_allocator::MMAllocator, penalties::AffinePenalties};

use sequence_distance_benchmark::test_data::test::Data;
use rust_hamming_distance::hamming_distance::HammingDistancable;
use triple_accel::hamming;



// use lib::euler1; // function to profile

pub fn bench_external_libwfa(c: &mut Criterion) {

    let alloc = MMAllocator::new(BUFFER_SIZE_8M as u64);    

    let mut penalties = AffinePenalties {
        match_: 0,
        mismatch: 4,
        gap_opening: 6,
        gap_extension: 2,
    };

        
    let data = Data::generate(0.2, 0.1, 150, 50);
    c.bench_function("libwfa3", |b| b.iter(|| {
        data.zip_seq().for_each(|(query,reference)| { 
            black_box(sequence_distance_benchmark::alignment::libwfa(query, reference, &alloc, &mut penalties));
        });
    }));

    let data = Data::generate(0.2, 0.1, 30, 50);
    c.bench_function("libwfa2", |b| b.iter(|| {
        data.zip_seq().for_each(|(query,reference)| { 
            black_box(sequence_distance_benchmark::alignment::libwfa(query, reference, &alloc, &mut penalties));
        });
    }));
            
    let data = Data::generate(0.05, 0.01, 150, 50);
    c.bench_function("libwfa1", |b| b.iter(|| {
        data.zip_seq().for_each(|(query,reference)| { 
            black_box(sequence_distance_benchmark::alignment::libwfa(query, reference, &alloc, &mut penalties));
        });
    }));
}

pub fn compare_noerror(c: &mut Criterion) {
    use criterion::black_box;


    let alloc = MMAllocator::new(BUFFER_SIZE_8M as u64);    
    let data = Data::generate(0.0, 0.0, 150, 1_000);

    let mut penalties = AffinePenalties {
        match_: 0,
        mismatch: 4,
        gap_opening: 6,
        gap_extension: 2,
    };
    let pens = Penalties {
        mismatch_pen: 1,
        open_pen: 6,
        extd_pen: 2,
    };

    let samples = 100;

    // let datasets = vec![
    //     ("100_sim_150", Data::generate(0.0, 0.0, 150, samples)),
    //     ("95_sim_150", Data::generate(0.05, 0.01, 150, samples)),
    //     ("90_sim_150", Data::generate(0.1, 0.01, 150, samples)),
    //     ("80_sim_150", Data::generate(0.2, 0.05, 150, samples)),
    //     ("50_sim_150", Data::generate(0.5, 0.1, 150, samples)),
    //     ("100_sim_30", Data::generate(0.0, 0.0, 30, samples*5)),
    //     ("95_sim_30", Data::generate(0.05, 0.01, 30, samples*5)),
    //     ("90_sim_30", Data::generate(0.1, 0.01, 30, samples*5)),
    //     ("80_sim_30", Data::generate(0.2, 0.05, 30, samples*5)),
    //     ("50_sim_30", Data::generate(0.5, 0.1, 30, samples*5)),
    //     ("100_sim_10", Data::generate(0.0, 0.0, 10, samples*15)),
    //     ("95_sim_10", Data::generate(0.05, 0.01, 10, samples*15)),
    //     ("90_sim_10", Data::generate(0.1, 0.01, 10, samples*15)),
    //     ("80_sim_10", Data::generate(0.2, 0.05, 10, samples*15)),
    //     ("50_sim_10", Data::generate(0.5, 0.1, 10, samples*15)),
    // ];

    // // let mut group: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> = c.benchmark_group("name");
    // let mut group: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> = c.benchmark_group("sim_len_variation");
    // for (id, (name, data)) in datasets.iter().enumerate() {
    //     group.bench_with_input(BenchmarkId::new("libwfa", id), &data, |b, data| b.iter(|| {
    //             data.zip_seq().for_each(|(query,reference)| { 
    //                 black_box(sequence_distance_benchmark::alignment::libwfa(query, reference, &alloc, &mut penalties));
    //             })
    //         })
    //     );
    //     group.bench_with_input(BenchmarkId::new("rust_wfa", id), &data, |b, data| b.iter(|| {
    //         data.zip_seq().for_each(|(query,reference)| { 
    //             let _ = black_box(wavefront_align(black_box(&String::from_utf8((query).to_vec()).unwrap()), black_box(&String::from_utf8((reference).to_vec()).unwrap()), black_box(&pens)));
    //         });
    //     }));
    // }
    // group.finish();


    let samples = 100;
    let datasets = vec![
        ("a150_90_sim", Data::generate(0.1, 0.01, 150, samples)),
        ("a140_90_sim", Data::generate(0.1, 0.01, 140, samples)),
        ("a130_90_sim", Data::generate(0.1, 0.01, 130, samples)),
        ("a120_90_sim", Data::generate(0.1, 0.01, 120, samples)),
        ("a110_90_sim", Data::generate(0.1, 0.01, 110, samples)),
        ("a100_90_sim", Data::generate(0.1, 0.01, 100, samples)),
        ("a090_90_sim", Data::generate(0.1, 0.01, 90, samples)),
        ("a070_90_sim", Data::generate(0.1, 0.01, 70, samples)),
        ("a080_90_sim", Data::generate(0.1, 0.01, 80, samples)),
        ("a060_90_sim", Data::generate(0.1, 0.01, 60, samples)),
        ("a050_90_sim", Data::generate(0.1, 0.01, 50, samples)),
        ("a040_90_sim", Data::generate(0.1, 0.01, 40, samples)),
        ("a030_90_sim", Data::generate(0.1, 0.01, 30, samples)),
        ("a020_90_sim", Data::generate(0.1, 0.01, 20, samples)),
        ("a010_90_sim", Data::generate(0.1, 0.01, 10, samples)),
        ("b050_90_sim", Data::generate(0.1, 0.01, 50, samples*3)),
        ("b010_90_sim_2", Data::generate(0.1, 0.01, 10, samples*15)),
    ];

    // let mut group: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> = c.benchmark_group("edit_vs_alignment");
    // for (id, (name, data)) in datasets.iter().enumerate() {
    //     group.bench_with_input(BenchmarkId::new("libwfa", id), &data, |b, data| b.iter(|| {
    //             data.zip_seq().for_each(|(query,reference)| { 
    //                 black_box(sequence_distance_benchmark::alignment::libwfa(query, reference, &alloc, &mut penalties));
    //             })
    //         })
    //     );
    //     group.bench_with_input(BenchmarkId::new("rust_wfa", id), &data, |b, data| b.iter(|| {
    //         data.zip_seq().for_each(|(query,reference)| { 
    //             let _ = black_box(wavefront_align(black_box(&String::from_utf8((query).to_vec()).unwrap()), black_box(&String::from_utf8((reference).to_vec()).unwrap()), black_box(&pens)));
    //         });
    //     }));
    //     group.bench_with_input(BenchmarkId::new("triple_accel_hamming", id), &data, |b, data| b.iter(|| {
    //         data.zip_seq().for_each(|(query,reference)| { 
    //             let len = min(query.len(), reference.len());
    //             black_box(triple_accel::hamming(&query[0..len], &reference[0..len])); 
    //         });
    //     }));
    //     group.bench_with_input(BenchmarkId::new("triple_accel_edit", id), &data, |b, data| b.iter(|| {
    //         data.zip_seq().for_each(|(query,reference)| {
    //             black_box(triple_accel::levenshtein(&query, &reference)); 
    //         });
    //     }));
    // }
    // group.finish();



    let mut group: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> = c.benchmark_group("hamming");

    for (id, (name, data)) in datasets.iter().enumerate() {
        let truth = data.zip_seq().map(|(query,reference)| {
            let len = min(query.len(), reference.len());
            hamming(&query[0..len], &reference[0..len]) as i32 }).collect::<Vec<i32>>();

        group.bench_with_input(BenchmarkId::new("hybrid_naive_triple_accel", id), &data, |b, data| b.iter(|| {
            data.zip_seq().enumerate().for_each(|(i, (query,reference))| { 
                let len = min(query.len(), reference.len());
                let dist = black_box(
                    if len <= 50 {
                        black_box(sequence_distance_benchmark::edit::local::hamming(&query[0..len], &reference[0..len]))
                    } else {
                        black_box(hamming(&query[0..len], &reference[0..len]) as i32)
                });
                assert_eq!(truth[i], dist);
            });
        }));
        group.bench_with_input(BenchmarkId::new("naive_hamming", id), &data, |b, data| b.iter(|| {
            data.zip_seq().enumerate().for_each(|(i, (query,reference))| { 
                let len = min(query.len(), reference.len());
                let dist = black_box(sequence_distance_benchmark::edit::local::hamming(&query[0..len], &reference[0..len]));
                assert_eq!(truth[i], dist);
            });
        }));
        group.bench_with_input(BenchmarkId::new("triple_accel::hamming", id), &data, |b, data| b.iter(|| {
            data.zip_seq().enumerate().for_each(|(i, (query,reference))| { 
                let len = min(query.len(), reference.len());
                let dist = black_box(hamming(&query[0..len], &reference[0..len]) as i32);
                assert_eq!(truth[i], dist);
            });
        }));
        group.bench_with_input(BenchmarkId::new("rust_hamming_distance::hamming", id), &data, |b, data| b.iter(|| {
            data.zip_seq().enumerate().for_each(|(i, (query,reference))| { 
                let len = min(query.len(), reference.len());
                let dist = black_box(query[0..len].hamming_distance(&reference[0..len]).unwrap() as i32);
                assert_eq!(truth[i], dist);
            });
        }));
        group.bench_with_input(BenchmarkId::new("wild!_naive_triple_accel", id), &data, |b, data| b.iter(|| {
            data.zip_seq().enumerate().for_each(|(i, (query,reference))| { 
                let len = min(query.len(), reference.len());
                let dist = black_box(sequence_distance_benchmark::edit::local::hamming(&query[0..len], &reference[0..len]));
                assert_eq!(truth[i], dist);
            });
        }));
    }
    group.finish();

}

criterion_group!(benches, compare_noerror);
criterion_main!(benches);