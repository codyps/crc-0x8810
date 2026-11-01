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
