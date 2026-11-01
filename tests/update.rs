#[test]
fn exhaustive_byte_update() {
    for seed in 0..=u16::MAX {
        for byte in 0..=u8::MAX {
            // Bitwise LSB-first division using the reflected 0x1021 polynomial.
            let mut expected = seed ^ u16::from(byte);
            for _ in 0..8 {
                expected = (expected >> 1) ^ if expected & 1 != 0 { 0x8408 } else { 0 };
            }
            assert_eq!(
                crc_0x8810::update(seed, byte),
                expected,
                "seed={seed:#06x}, byte={byte:#04x}"
            );
        }
    }
}

#[test]
fn exhaustive_msb_byte_update() {
    for seed in 0..=u16::MAX {
        let algorithm = crc_0x8810::Algorithm {
            init: seed,
            ..crc_0x8810::CRC_16_XMODEM
        };
        for byte in 0..=u8::MAX {
            // Independently divide one byte with the non-reflected polynomial.
            let mut expected = seed ^ (u16::from(byte) << 8);
            for _ in 0..8 {
                expected = (expected << 1) ^ if expected & 0x8000 != 0 { 0x1021 } else { 0 };
            }
            assert_eq!(
                algorithm.checksum(&[byte]),
                expected,
                "seed={seed:#06x}, byte={byte:#04x}"
            );
        }
    }
}
