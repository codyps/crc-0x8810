use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

fn benchmark_input() -> [u8; 4096] {
    // Cover every byte value reproducibly without an RNG dependency.
    std::array::from_fn(|i| (i as u8).wrapping_mul(31).wrapping_add(17))
}

fn crc_xmodem(c: &mut Criterion) {
    let mut group = c.benchmark_group("xmodem");
    let buf = benchmark_input();
    group.bench_function("crc-ccitt", |b| {
        b.iter(|| crc_0x8810::CRC_16_XMODEM.checksum(black_box(&buf)))
    });

    group.bench_function("crc-any", |b| {
        b.iter(|| {
            let mut crc_any = crc_any::CRCu16::crc16xmodem();
            crc_any.digest(black_box(&buf));
            black_box(crc_any.get_crc())
        })
    });

    let crc = crc::Crc::<u16>::new(&crc::CRC_16_XMODEM);
    group.bench_function("crc", |b| {
        b.iter(|| {
            let mut digest = crc.digest();
            digest.update(black_box(&buf));
            black_box(digest.finalize());
        })
    });
}

fn crc_kermit(c: &mut Criterion) {
    let mut group = c.benchmark_group("kermit");
    let buf = benchmark_input();
    group.bench_function("crc-ccitt", |b| {
        b.iter(|| crc_0x8810::CRC_16_KERMIT.checksum(black_box(&buf)))
    });

    group.bench_function("crc-any", |b| {
        b.iter(|| {
            let mut crc_any = crc_any::CRCu16::crc16kermit();
            crc_any.digest(black_box(&buf));
            black_box(crc_any.get_crc())
        })
    });

    let crc = crc::Crc::<u16>::new(&crc::CRC_16_KERMIT);
    group.bench_function("crc", |b| {
        b.iter(|| {
            let mut digest = crc.digest();
            digest.update(black_box(&buf));
            black_box(digest.finalize());
        })
    });
}

criterion_group!(benches, crc_kermit, crc_xmodem);
criterion_main!(benches);
