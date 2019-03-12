#![deny(warnings)]

use neqo_crypto::constants::*;
use neqo_crypto::hpmask::hpmask;
use neqo_crypto::{init_db, SymKey, SymKeyTarget};

#[test]
fn aes_ecb() {
    init_db("./db");
    let key = SymKey::import(
        SymKeyTarget::HpMask(TLS_AES_128_GCM_SHA256),
        &[
            0x94, 0xb9, 0x45, 0x2d, 0x2b, 0x3c, 0x7c, 0x7f, 0x6d, 0xa7, 0xfd, 0xd8, 0x59, 0x35,
            0x37, 0xfd,
        ],
    )
    .expect("can make key");
    let input = &[
        0xc4, 0xc2, 0xa2, 0x30, 0x3d, 0x29, 0x7e, 0x3c, 0x51, 0x9b, 0xf6, 0xb2, 0x23, 0x86, 0xe3,
        0xd0,
    ];
    let mask = hpmask(&key, input).expect("should produce a mask");
    const EXPECTED: &[u8] = &[
        0x75, 0xf7, 0xec, 0x8b, 0x62, 0x20, 0xac, 0x7f, 0x30, 0x57, 0xec, 0x8e, 0x38, 0x25, 0xe2,
        0x8f,
    ];
    assert_eq!(mask, EXPECTED);
}

#[cfg(feature = "chacha")]
#[test]
fn chacha20_ctr() {
    init_db("./db");
    let key = SymKey::import(
        SymKeyTarget::HpMask(TLS_CHACHA20_POLY1305_SHA256),
        &[
            0x30, 0x82, 0x02, 0x72, 0x02, 0x01, 0x00, 0x30, 0x0d, 0x06, 0x09, 0x2a, 0x86, 0x48,
            0x86, 0xf7, 0x0d, 0x01, 0x01, 0x01, 0x05, 0x00, 0x04, 0x82, 0x02, 0x5c, 0x30, 0x82,
            0x02, 0x58, 0x02, 0x01,
        ],
    )
    .expect("can make key");
    let input = &[
        0xc4, 0xc2, 0xa2, 0x30, 0x3d, 0x29, 0x7e, 0x3c, 0x51, 0x9b, 0xf6, 0xb2, 0x23, 0x86, 0xe3,
        0xd0,
    ];
    let mask = hpmask(&key, input).expect("should produce a mask");
    const EXPECTED: &[u8] = &[
        0xb3, 0x9e, 0xb8, 0xa5, 0x9b, 0x37, 0x25, 0x2a, 0xcd, 0xc2, 0xae, 0x9d, 0x91, 0x2c, 0xff,
        0x3f, 0x24, 0x32, 0xc5, 0xbb, 0x5d, 0x32, 0xa5, 0xed, 0x1a, 0x30, 0x7a, 0x6e, 0xec, 0x7f,
        0x72, 0xb7, 0x5e, 0x67, 0xc6, 0x7a, 0x8c, 0x08, 0xb3, 0xbd, 0x8a, 0xf1, 0xef, 0x74, 0x1d,
        0x86, 0x55, 0x3e, 0x0e, 0x60, 0x43, 0xc4, 0xd2, 0xd5, 0x92, 0xe9, 0xe0, 0x25, 0xa7, 0xfb,
        0xc7, 0x0b, 0xb8, 0xfa,
    ];
    assert_eq!(mask, EXPECTED);
}
