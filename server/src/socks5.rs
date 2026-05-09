pub mod config;
pub mod connection;
pub mod atyp;
pub mod parse;

pub mod consts {
    pub const SOCKS_VERSION: u8 = 0x05;
    pub const NO_AUTH: u8 = 0x00;
    pub const AUTH: u8 = 0x02;
    pub const AUTH_VERSION: u8 = 0x01;

    pub const CMD_CONNECT: u8 = 0x01;
    
    pub mod reply {
        pub const SUCCESS: u8 = 0x00;
        pub const FAILURE: u8 = 0x01;
        pub const NO_ACCEPTABLE_METHOD: u8 = 0xFF;

        pub const RSV: u8 = 0x00;
        pub const BND_ADDR: &[u8] = &[0x00, 0x00, 0x00, 0x00];
        pub const BND_PORT: &[u8] = &[0x00, 0x00];
    }
}