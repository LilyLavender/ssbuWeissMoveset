use super::*;

pub mod statusglyph;
pub mod statusmove;

pub fn install() {
    statusglyph::install();
    statusmove::install();
}