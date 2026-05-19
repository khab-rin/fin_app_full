pub mod primitives_re;
pub mod const_var;
pub mod crypto_re;

use crate::static_data::primitives_re::init_primitivrs_re;
use crate::static_data::crypto_re::init_crypto_re;

pub fn init_re() {
    init_primitivrs_re();
    init_crypto_re();
}

