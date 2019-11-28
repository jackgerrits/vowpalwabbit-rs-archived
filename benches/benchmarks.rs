#[macro_use]
extern crate bencher;

use bencher::Bencher;

use vowpalwabbit;

fn uniform_hash_10chars(bench: &mut Bencher) {
    bench.iter(|| vowpalwabbit::hash::uniform_hash(b"abcdefghij", 0))
}

fn uniform_hash_100chars(bench: &mut Bencher) {
    bench.iter(|| {
        vowpalwabbit::hash::uniform_hash(b"abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij", 0)
    })
}

benchmark_group!(benches, uniform_hash_10chars, uniform_hash_100chars);
benchmark_main!(benches);
