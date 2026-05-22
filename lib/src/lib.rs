use uint::construct_uint;

construct_uint! {
    // construit un entier non-signé de 256 bits
    // constitué de mot de 4 x 64 bits
    pub struct U256(4);
}

pub mod crypto;
pub mod sha256;
pub mod types;
pub mod util;
