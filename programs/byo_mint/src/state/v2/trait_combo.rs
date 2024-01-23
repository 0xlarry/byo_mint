use crate::*;

#[account]
pub struct TraitCombo {
    pub variants: [u8; 10]
}

impl TraitCombo {
    pub const LEN: usize = 8
        + 1 * 10;
}