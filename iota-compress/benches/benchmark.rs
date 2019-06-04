#[macro_use]
extern crate criterion;
extern crate rand;

use criterion::Criterion;
use iota_compress::{compress, decompress};
use rand::Rng;

const PACKET_SIZE: usize = 2673;

fn generate_random() -> String {
    let tryte_alphabet: Vec<char> = vec!['9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];

    let mut output: String = String::new();

    for _ in 0..PACKET_SIZE {
        let num = rand::thread_rng().gen_range(0, 27);
        output.push(tryte_alphabet[num]);
    }

    output
}

fn test_compress(trytes: &String) {
    compress(&trytes);
} 

fn test_decompress(compressed: &Vec<u8>) {
    decompress(compressed.to_vec());
}

fn criterion_benchmark(c: &mut Criterion) {
    let trytes = generate_random();
    let compressed = compress(&trytes);
    c.bench_function("Compress", move |b| b.iter(|| test_compress(&trytes)));
    c.bench_function("Deompress", move |b| b.iter(|| test_decompress(&compressed)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);