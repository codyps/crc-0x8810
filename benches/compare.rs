use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::RngCore;

fn crc_xmodem(c: &mut Criterion) {
    let mut group = c.benchmark_group("xmodem");
    let mut rng = rand::thread_rng();
    let mut buf = [0u8; 4096];
    rng.fill_bytes(&mut buf);
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
    let mut rng = rand::thread_rng();
    let mut buf = [0u8; 4096];
    rng.fill_bytes(&mut buf);
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
