use crc_0x8810::*;

// Exercise const evaluation from a downstream crate, where inlining matters.
const CHECKSUMS: [u16; 7] = [
    CRC_16_XMODEM.checksum(b"123456789"),
    CRC_16_GENIBUS.checksum(b"123456789"),
    CRC_16_GSM.checksum(b"123456789"),
    CRC_16_IBM_3740.checksum(b"123456789"),
    CRC_16_IBM_SDLC.checksum(b"123456789"),
    CRC_16_ISO_IEC_14443_3_A.checksum(b"123456789"),
    CRC_16_KERMIT.checksum(b"123456789"),
];

#[test]
fn const_checksums() {
    assert_eq!(
        CHECKSUMS,
        [0x31c3, 0xd64e, 0xce3c, 0x29b1, 0x906e, 0xbf05, 0x2189]
    );
}

#[test]
fn cloned_digests_can_continue_independently() {
    for algorithm in [
        CRC_16_XMODEM,
        CRC_16_GENIBUS,
        CRC_16_GSM,
        CRC_16_IBM_3740,
        CRC_16_IBM_SDLC,
        CRC_16_ISO_IEC_14443_3_A,
        CRC_16_KERMIT,
    ] {
        let mut prefix = algorithm.digest();
        prefix.update(b"1234");
        for suffix in [b"".as_slice(), b"56789", b"\x00\xff\x80"] {
            let mut digest = prefix.clone();
            digest.update(b"");
            for byte in suffix.chunks(1) {
                digest.update(byte);
            }
            let mut bytes = b"1234".to_vec();
            bytes.extend_from_slice(suffix);
            assert_eq!(digest.finalize(), algorithm.checksum(&bytes));
        }
        assert_eq!(prefix.finalize(), algorithm.checksum(b"1234"));
    }
}
