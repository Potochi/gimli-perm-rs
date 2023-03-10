pub const GIMLI_ROWS: usize = 3;
pub const GIMLI_COLS: usize = 4;
pub const GIMLI_XOR_CONSTANT: u32 = 0x9e377900;
pub const GIMLI_SIZE: usize = 12;

pub const GIMLI_REF_INPUT: [u32; GIMLI_SIZE] = [
    0x00000000, 0x9e3779ba, 0x3c6ef37a, 0xdaa66d46, 0x78dde724, 0x1715611a, 0xb54cdb2e, 0x53845566,
    0xf1bbcfc8, 0x8ff34a5a, 0x2e2ac522, 0xcc624026,
];

pub const GIMLI_REF_OUTPUT: [u32; GIMLI_SIZE] = [
    0xba11c85a, 0x91bad119, 0x380ce880, 0xd24c2c68, 0x3eceffea, 0x277a921c, 0x4f73a0bd, 0xda5a9cd8,
    0x84b673f0, 0x34e52ff7, 0x9e2bef49, 0xf41bb8d6,
];
