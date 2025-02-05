use super::*;

pub mod specialn;
pub mod specials;
pub mod speciallw;

pub fn install() {
    specialn::install();
    specials::install();
    speciallw::install();
}