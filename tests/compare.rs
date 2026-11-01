use proptest::prelude::*;

#[test]
fn plain() {
    let crc_a = crc::Crc::<u16>::new(&crc::CRC_16_XMODEM);
    let crc_b = crc_0x8810::CRC_16_XMODEM;

    proptest!(move |(s: Vec<u8>)| {
        let a = {
            let mut d = crc_a.digest();
            d.update(&s[..]);
            d.finalize()
        };
        let b = crc_b.checksum(&s[..]);
        prop_assert_eq!(a, b);
    })
}

const SPI_FUJITSU: crc_0x8810::Algorithm = crc_0x8810::Algorithm {
    init: 0x1d0f,
    refin: false,
    refout: false,
    xorout: 0,
    check: 0xe5cc,
    residue: 0,
};

#[test]
fn non_reflected_seed_checksum() {
    const CHECKSUM: u16 = SPI_FUJITSU.checksum(b"123456789");
    assert_eq!(CHECKSUM, 0xe5cc);
    assert_eq!(SPI_FUJITSU.checksum(b""), 0x1d0f);
}

#[test]
fn non_reflected_seed_digest() {
    assert_eq!(SPI_FUJITSU.digest().finalize(), 0x1d0f);
    for split in 0..=9 {
        let mut digest = SPI_FUJITSU.digest();
        digest.update(&b"123456789"[..split]);
        digest.update(b"");
        digest.update(&b"123456789"[split..]);
        assert_eq!(digest.finalize(), 0xe5cc);
    }
}

#[test]
fn asymmetric_seed_reflection_combinations() {
    static REFERENCES: [crc::Algorithm<u16>; 4] = [
        crc::Algorithm {
            refin: false,
            refout: false,
            ..crc::CRC_16_SPI_FUJITSU
        },
        crc::Algorithm {
            refin: false,
            refout: true,
            ..crc::CRC_16_SPI_FUJITSU
        },
        crc::Algorithm {
            refin: true,
            refout: false,
            ..crc::CRC_16_SPI_FUJITSU
        },
        crc::Algorithm {
            refin: true,
            refout: true,
            ..crc::CRC_16_SPI_FUJITSU
        },
    ];

    proptest!(|(bytes: Vec<u8>, split in any::<usize>(), xorout in any::<u16>())| {
        let split = split % (bytes.len() + 1);
        for reference in &REFERENCES {
            let algorithm = crc_0x8810::Algorithm {
                refin: reference.refin,
                refout: reference.refout,
                xorout,
                ..SPI_FUJITSU
            };
            let expected = crc::Crc::<u16>::new(reference).checksum(&bytes) ^ xorout;
            prop_assert_eq!(algorithm.checksum(&bytes), expected);
            let mut digest = algorithm.digest();
            digest.update(&bytes[..split]);
            digest.update(b"");
            digest.update(&bytes[split..]);
            prop_assert_eq!(digest.finalize(), expected);
        }
    });
}

// Independent MSB-first bitwise reference, using the public parameter convention.
fn bitwise_checksum(algorithm: &crc_0x8810::Algorithm, bytes: &[u8]) -> u16 {
    let mut crc = algorithm.init;
    for &byte in bytes {
        let byte = if algorithm.refin {
            byte.reverse_bits()
        } else {
            byte
        };
        crc ^= u16::from(byte) << 8;
        for _ in 0..8 {
            crc = (crc << 1) ^ if crc & 0x8000 != 0 { 0x1021 } else { 0 };
        }
    }
    if algorithm.refout {
        crc = crc.reverse_bits();
    }
    crc ^ algorithm.xorout
}

#[test]
fn arbitrary_parameters_match_bitwise_reference() {
    proptest!(|(bytes: Vec<u8>, init in any::<u16>(), xorout in any::<u16>(), split in any::<usize>())| {
        let split = split % (bytes.len() + 1);
        for refin in [false, true] {
            for refout in [false, true] {
                let algorithm = crc_0x8810::Algorithm {
                    init,
                    refin,
                    refout,
                    xorout,
                    ..SPI_FUJITSU
                };
                let expected = bitwise_checksum(&algorithm, &bytes);
                prop_assert_eq!(algorithm.checksum(&bytes), expected);
                // Check initialization and finalization with no input as well.
                prop_assert_eq!(algorithm.checksum(b""), bitwise_checksum(&algorithm, b""));
                let mut digest = algorithm.digest();
                digest.update(&bytes[..split]);
                digest.update(b"");
                digest.update(&bytes[split..]);
                prop_assert_eq!(digest.finalize(), expected);
                prop_assert_eq!(algorithm.digest().finalize(), bitwise_checksum(&algorithm, b""));
            }
        }
    });
}

#[test]
fn crc_any_reference_algorithms() {
    let all_bytes: Vec<u8> = (0..=u8::MAX).collect();
    for bytes in [b"".as_slice(), b"123456789", &all_bytes] {
        for (algorithm, mut reference) in [
            (crc_0x8810::CRC_16_XMODEM, crc_any::CRCu16::crc16xmodem()),
            (crc_0x8810::CRC_16_KERMIT, crc_any::CRCu16::crc16kermit()),
        ] {
            reference.digest(bytes);
            assert_eq!(algorithm.checksum(bytes), reference.get_crc());
        }
    }
}
