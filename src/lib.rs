#![no_std]
#![forbid(unsafe_code)]

//! compute crcs using the ccitt polynomial efficiently without tables
//!
//! P(x) = x**16 + x**12 + x**5 + 1
//!
//! MSB polynomial: 0x8810
//! MSB polynomial (with explicit 1): 0x1021
//!
//! <https://users.ece.cmu.edu/~koopman/crc/c16/0x8810.txt>
//!
//! The method used is described in a few places:
//!
//!  - [Greg Cook provides some commented 6502 asm (6502.org source)](https://github.com/6502org/6502.org/blob/main/public/source/integers/crc-more.html)
//!  - [Jon Buller describes how to determine it through automated symbolic calculation and provides some 8051 asm](https://groups.google.com/g/comp.arch.embedded/c/fvQ7yM5F6ys/m/3xcgqF3Kqc4J)
//!  - [adapted by others into C](https://www.ccsinfo.com/forum/viewtopic.php?t=24977)
//!  - the same method is used in [avr-libc](https://avrdudes.github.io/avr-libc/avr-libc-user-manual/group__util__crc.html)'s `_crc_ccitt_update` function.
//!

/// The lowest level operation, applies a single byte of data to a given crc and returns the new
/// crc
///
/// NOTE: internally, this is performing a least significant bit (LSB) first crc. This means that
/// performing an MSB first operation requires reversing the bits of each input byte and revsersing
/// the final CRC output. It _may_ be possible to determine a direct method to calculate the crc
/// without needing a reverse bits operation (which can be expensive on some platforms)
#[inline]
pub const fn update(crc: u16, data: u8) -> u16 {
    let data = data ^ (crc as u8);
    let data = data ^ (data << 4);
    (((data as u16) << 8) | (crc >> 8)) ^ ((data >> 4) as u16) ^ ((data as u16) << 3)
}

#[derive(Debug, Copy, Clone)]
pub struct Algorithm {
    pub init: u16,
    pub refin: bool,
    pub refout: bool,
    pub xorout: u16,
    pub check: u16,
    pub residue: u16,
}

impl Algorithm {
    #[inline]
    pub const fn checksum(&self, bytes: &[u8]) -> u16 {
        let mut crc = self.init();
        crc = self.update(crc, bytes);
        self.finalize(crc)
    }

    #[inline]
    const fn init(&self) -> u16 {
        // The byte update always uses an LSB-first register, regardless of refin.
        self.init.reverse_bits()
    }

    #[inline]
    const fn update(&self, mut crc: u16, bytes: &[u8]) -> u16 {
        let mut i = 0;
        if self.refin {
            while i < bytes.len() {
                crc = update(crc, bytes[i]);
                i += 1;
            }
        } else {
            while i < bytes.len() {
                crc = update(crc, bytes[i].reverse_bits());
                i += 1;
            }
        }
        crc
    }

    #[inline]
    const fn finalize(&self, mut crc: u16) -> u16 {
        if !self.refout {
            crc = crc.reverse_bits();
        }
        crc ^ self.xorout
    }

    #[inline]
    pub const fn digest(&self) -> Digest {
        Digest::new(self)
    }
}

/// A `crc` crate like `Digest` api
#[derive(Debug, Copy, Clone)]
pub struct Digest<'a> {
    algorithm: &'a Algorithm,
    value: u16,
}

impl<'a> Digest<'a> {
    #[inline]
    const fn new(algorithm: &'a Algorithm) -> Self {
        let value = algorithm.init();
        Digest { algorithm, value }
    }

    #[inline]
    pub fn update(&mut self, bytes: &[u8]) {
        self.value = self.algorithm.update(self.value, bytes);
    }

    #[inline]
    pub const fn finalize(self) -> u16 {
        self.algorithm.finalize(self.value)
    }
}

/// CRC-16/XMODEM
///
/// width=16 poly=0x1021 init=0x0000 refin=false refout=false xorout=0x0000 check=0x31c3 residue=0x0000 name="CRC-16/XMODEM"
pub const CRC_16_XMODEM: Algorithm = Algorithm {
    init: 0,
    refin: false,
    refout: false,
    xorout: 0,
    check: 0x31c3,
    residue: 0,
};

/// CRC-16/GENIBUS
///
/// width=16 poly=0x1021 init=0xffff refin=false refout=false xorout=0xffff check=0xd64e residue=0x1d0f name="CRC-16/GENIBUS"
pub const CRC_16_GENIBUS: Algorithm = Algorithm {
    init: 0xffff,
    refin: false,
    refout: false,
    xorout: 0xffff,
    check: 0xd64e,
    residue: 0x1d0f,
};

/// CRC-16/GSM
///
/// width=16 poly=0x1021 init=0x0000 refin=false refout=false xorout=0xffff check=0xce3c residue=0x1d0f name="CRC-16/GSM"
pub const CRC_16_GSM: Algorithm = Algorithm {
    init: 0,
    refin: false,
    refout: false,
    xorout: 0xffff,
    check: 0xce3c,
    residue: 0x1d0f,
};

/// CRC-16/IBM-3740
///
/// width=16 poly=0x1021 init=0xffff refin=false refout=false xorout=0x0000 check=0x29b1 residue=0x0000 name="CRC-16/IBM-3740"
pub const CRC_16_IBM_3740: Algorithm = Algorithm {
    init: 0xffff,
    refin: false,
    refout: false,
    xorout: 0,
    check: 0x29b1,
    residue: 0x000,
};

/// Alias of [`CRC_16_IBM_3740`]
pub const CRC_16_AUTOSAR: Algorithm = CRC_16_IBM_3740;

/// CRC-16/IBM-SDLC
///
/// width=16 poly=0x1021 init=0xffff refin=true refout=true xorout=0xffff check=0x906e residue=0xf0b8 name="CRC-16/IBM-SDLC"
pub const CRC_16_IBM_SDLC: Algorithm = Algorithm {
    init: 0xffff,
    refin: true,
    refout: true,
    xorout: 0xffff,
    check: 0x906e,
    residue: 0xf0b8,
};

/// Alias of [`CRC_16_IBM_SDLC`]
pub const CRC_16_ISO_HDLC: Algorithm = CRC_16_IBM_SDLC;
/// Alias of [`CRC_16_IBM_SDLC`]
pub const CRC_16_ISO_IEC_14443_3_B: Algorithm = CRC_16_IBM_SDLC;
/// Alias of [`CRC_16_IBM_SDLC`]
pub const CRC_16_X_25: Algorithm = CRC_16_IBM_SDLC;

/// CRC-16/ISO-IEC-14443-3-A
///
/// width=16 poly=0x1021 init=0xc6c6 refin=true refout=true xorout=0x0000 check=0xbf05 residue=0x0000 name="CRC-16/ISO-IEC-14443-3-A"
pub const CRC_16_ISO_IEC_14443_3_A: Algorithm = Algorithm {
    init: 0xc6c6,
    refin: true,
    refout: true,
    xorout: 0,
    check: 0xbf05,
    residue: 0,
};

/// CRC-16/KERMIT
///
/// width=16 poly=0x1021 init=0x0000 refin=true refout=true xorout=0x0000 check=0x2189 residue=0x0000 name="CRC-16/KERMIT"
pub const CRC_16_KERMIT: Algorithm = Algorithm {
    init: 0,
    refin: true,
    refout: true,
    xorout: 0,
    check: 0x2189,
    residue: 0,
};

/// Alias of [`CRC_16_KERMIT`]
pub const CRC_16_CCITT: Algorithm = CRC_16_KERMIT;

/// Alias of [`CRC_16_XMODEM`]
pub const CRC_16_LORA: Algorithm = CRC_16_XMODEM;
