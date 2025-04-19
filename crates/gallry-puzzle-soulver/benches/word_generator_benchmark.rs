use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use gallry_puzzle_soulver::{Slot, WordGenerator};
use std::collections::HashSet;

fn generate_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("word_generation");

    // Benchmark different numbers of slots and options
    for slot_count in [2, 3, 4, 5] {
        for option_count in [3, 5, 8] {
            // Create slots with varying numbers of character options
            let slots = (0..slot_count)
                .map(|_| {
                    let options = (0..option_count)
                        .map(|i| (b'a' + (i % 26) as u8) as char)
                        .collect();
                    Slot::new(options)
                })
                .collect::<Vec<_>>();

            // Benchmark with no filtering
            group.bench_with_input(
                BenchmarkId::new(
                    "no_filter",
                    format!("slots={},options={}", slot_count, option_count),
                ),
                &slots,
                |b, slots| {
                    b.iter(|| {
                        let generator = WordGenerator::with_no_filtering(slots.clone());
                        generator.iter().count()
                    })
                },
            );

            // Benchmark with default word list filtering
            group.bench_with_input(
                BenchmarkId::new(
                    "with_filter",
                    format!("slots={},options={}", slot_count, option_count),
                ),
                &slots,
                |b, slots| {
                    b.iter(|| {
                        let generator = WordGenerator::with_slots(slots.clone());
                        generator.iter().count()
                    })
                },
            );
        }
    }

    group.finish();
}

fn filter_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("word_filtering");

    // Create a standard set of slots for testing filtering performance
    let slots = vec![
        Slot::new(vec!['c', 'b', 'r']),
        Slot::new(vec!['a', 'i', 'o']),
        Slot::new(vec!['t', 's', 'e']),
    ];

    // Benchmark filtering with different word list sizes
    for word_count in [10, 100, 1000, 10000] {
        // Create a custom word list of specified size for benchmarking
        let word_list: HashSet<String> = (0..word_count).map(|i| format!("word{}", i)).collect();

        group.bench_with_input(
            BenchmarkId::new("custom_wordlist", format!("words={}", word_count)),
            &word_list,
            |b, word_list| {
                b.iter(|| {
                    let generator = WordGenerator::new(slots.clone(), Some(word_list.clone()));
                    generator.iter().count()
                })
            },
        );
    }

    // Also benchmark with default wordlist
    group.bench_function("default_wordlist", |b| {
        b.iter(|| {
            let generator = WordGenerator::with_slots(slots.clone());
            generator.iter().count()
        })
    });

    group.finish();
}

criterion_group!(benches, generate_benchmark, filter_benchmark);
criterion_main!(benches);
