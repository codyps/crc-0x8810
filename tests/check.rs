use crc_0x8810::*;

fn check(algorithm: &Algorithm) {
    let actual = algorithm.checksum(b"123456789");
    assert_eq!(
        actual, algorithm.check,
        "checksum {actual:#06x}, expected {:#06x}",
        algorithm.check
    );
}

#[test]
fn crc_16_xmodem() {
    check(&CRC_16_XMODEM);
}

#[test]
fn crc_16_genibus() {
    check(&CRC_16_GENIBUS);
}

#[test]
fn crc_16_gsm() {
    check(&CRC_16_GSM);
}

#[test]
fn crc_16_ibm_3740() {
    check(&CRC_16_IBM_3740);
}

#[test]
fn crc_16_ibm_sdlc() {
    check(&CRC_16_IBM_SDLC);
}

#[test]
fn crc_16_iso_iec_1443_3_a() {
    check(&CRC_16_ISO_IEC_14443_3_A);
}

#[test]
fn crc_16_kermit() {
    check(&CRC_16_KERMIT);
}
