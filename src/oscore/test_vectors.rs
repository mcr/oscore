// RFC 8613 test vectors & examples ---------------------------------------

// AAD example

pub const EXAMPLE_KID: [u8; 1] = [0x00];
pub const EXAMPLE_PIV: [u8; 1] = [0x25];
pub const EXAMPLE_AAD_ARR: [u8; 9] =
    [0x85, 0x01, 0x81, 0x0A, 0x41, 0x00, 0x41, 0x25, 0x40];
pub const EXAMPLE_AAD: [u8; 21] = [
    0x83, 0x68, 0x45, 0x6E, 0x63, 0x72, 0x79, 0x70, 0x74, 0x30, 0x40, 0x49,
    0x85, 0x01, 0x81, 0x0A, 0x41, 0x00, 0x41, 0x25, 0x40,
];

// COSE compression (OSCORE option) examples

pub const EX1_KID: Option<&[u8]> = Some(&[0x25]);
pub const EX1_PIV: Option<&[u8]> = Some(&[0x05]);
pub const EX1_OPTION: [u8; 3] = [0x09, 0x05, 0x25];
pub const EX2_KID: Option<&[u8]> = Some(&[]);
pub const EX2_PIV: Option<&[u8]> = Some(&[0x00]);
pub const EX2_OPTION: [u8; 2] = [0x09, 0x00];
pub const EX4_KID: Option<&[u8]> = None;
pub const EX4_PIV: Option<&[u8]> = None;
pub const EX4_OPTION: [u8; 0] = [];
pub const EX5_KID: Option<&[u8]> = None;
pub const EX5_PIV: Option<&[u8]> = Some(&[0x07]);
pub const EX5_OPTION: [u8; 2] = [0x01, 0x07];

// Test vector 1

pub const MASTER_SECRET: [u8; 16] = [
    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C,
    0x0D, 0x0E, 0x0F, 0x10,
];
pub const MASTER_SALT: [u8; 8] =
    [0x9E, 0x7C, 0xA9, 0x22, 0x23, 0x78, 0x63, 0x40];
pub const CLIENT_ID: [u8; 0] = [];
pub const SERVER_ID: [u8; 1] = [0x01];
pub const INFO_CLIENT_KEY: [u8; 9] =
    [0x85, 0x40, 0xF6, 0x0A, 0x63, 0x4B, 0x65, 0x79, 0x10];
pub const INFO_SERVER_KEY: [u8; 10] =
    [0x85, 0x41, 0x01, 0xF6, 0x0A, 0x63, 0x4B, 0x65, 0x79, 0x10];
pub const INFO_COMMON_IV: [u8; 8] =
    [0x85, 0x40, 0xF6, 0x0A, 0x62, 0x49, 0x56, 0x0D];
pub const CLIENT_KEY: [u8; 16] = [
    0xF0, 0x91, 0x0E, 0xD7, 0x29, 0x5E, 0x6A, 0xD4, 0xB5, 0x4F, 0xC7, 0x93,
    0x15, 0x43, 0x02, 0xFF,
];
pub const SERVER_KEY: [u8; 16] = [
    0xFF, 0xB1, 0x4E, 0x09, 0x3C, 0x94, 0xC9, 0xCA, 0xC9, 0x47, 0x16, 0x48,
    0xB4, 0xF9, 0x87, 0x10,
];
pub const COMMON_IV: [u8; 13] = [
    0x46, 0x22, 0xD4, 0xDD, 0x6D, 0x94, 0x41, 0x68, 0xEE, 0xFB, 0x54, 0x98,
    0x7C,
];
pub const CLIENT_NONCE: [u8; 13] = [
    0x46, 0x22, 0xD4, 0xDD, 0x6D, 0x94, 0x41, 0x68, 0xEE, 0xFB, 0x54, 0x98,
    0x68,
];
pub const SERVER_NONCE: [u8; 13] = [
    0x47, 0x22, 0xD4, 0xDD, 0x6D, 0x94, 0x41, 0x69, 0xEE, 0xFB, 0x54, 0x98,
    0x7C,
];

// Test vector 4 (uses context from test vector 1)

pub const REQ_UNPROTECTED: [u8; 22] = [
    0x44, 0x01, 0x5D, 0x1F, 0x00, 0x00, 0x39, 0x74, 0x39, 0x6C, 0x6F, 0x63,
    0x61, 0x6C, 0x68, 0x6F, 0x73, 0x74, 0x83, 0x74, 0x76, 0x31,
];
pub const REQ_SSN: u64 = 20;
pub const REQ_PIV: [u8; 1] = [0x14];
pub const REQ_AAD_ARR: [u8; 8] =
    [0x85, 0x01, 0x81, 0x0A, 0x40, 0x41, 0x14, 0x40];
pub const REQ_AAD: [u8; 20] = [
    0x83, 0x68, 0x45, 0x6E, 0x63, 0x72, 0x79, 0x70, 0x74, 0x30, 0x40, 0x48,
    0x85, 0x01, 0x81, 0x0A, 0x40, 0x41, 0x14, 0x40,
];
pub const REQ_PROTECTED: [u8; 35] = [
    0x44, 0x02, 0x5D, 0x1F, 0x00, 0x00, 0x39, 0x74, 0x39, 0x6C, 0x6F, 0x63,
    0x61, 0x6C, 0x68, 0x6F, 0x73, 0x74, 0x62, 0x09, 0x14, 0xFF, 0x61, 0x2F,
    0x10, 0x92, 0xF1, 0x77, 0x6F, 0x1C, 0x16, 0x68, 0xB3, 0x82, 0x5E,
];

// Test vector 7 (uses context from test vector 1 & parts from vector 4)

pub const RES_UNPROTECTED: [u8; 21] = [
    0x64, 0x45, 0x5D, 0x1F, 0x00, 0x00, 0x39, 0x74, 0xFF, 0x48, 0x65, 0x6C,
    0x6C, 0x6F, 0x20, 0x57, 0x6F, 0x72, 0x6C, 0x64, 0x21,
];
pub const RES_PROTECTED: [u8; 32] = [
    0x64, 0x44, 0x5D, 0x1F, 0x00, 0x00, 0x39, 0x74, 0x90, 0xFF, 0xDB, 0xAA,
    0xD1, 0xE9, 0xA7, 0xE7, 0xB2, 0xA8, 0x13, 0xD3, 0xC3, 0x15, 0x24, 0x37,
    0x83, 0x03, 0xCD, 0xAF, 0xAE, 0x11, 0x91, 0x06,
];

// Test vector 8 (like test vector 7, but with PIV)

pub const RES_PIV: [u8; 1] = [0x00];
pub const RES_PIV_PROTECTED: [u8; 34] = [
    0x64, 0x44, 0x5D, 0x1F, 0x00, 0x00, 0x39, 0x74, 0x92, 0x01, 0x00, 0xFF,
    0x4D, 0x4C, 0x13, 0x66, 0x93, 0x84, 0xB6, 0x73, 0x54, 0xB2, 0xB6, 0x17,
    0x5F, 0xF4, 0xB8, 0x65, 0x8C, 0x66, 0x6A, 0x6C, 0xF8, 0x8E,
];

// Custom test vectors ----------------------------------------------------

pub const CRASH_OPTION: [u8; 2] = [0b0000_1101, 0x01];
pub const SERVER_NONCE_LONG_PIV: [u8; 13] = [
    0x41, 0x22, 0xD4, 0xDD, 0x6D, 0x94, 0x41, 0x69, 0xEE, 0xFB, 0x54, 0x98,
    0x7C,
];
pub const SERVER_ID_LONG: [u8; 10] =
    [0x01, 0x02, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01];
