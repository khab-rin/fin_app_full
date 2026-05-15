pub mod static_re;
pub(crate) mod const_var;
pub(crate) use crate::static_data::static_re::init_static_regex;





pub fn init() {
    init_static_regex();
}