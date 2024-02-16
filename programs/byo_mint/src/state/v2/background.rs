use crate::*;

#[account]
pub struct Background {
    pub color_or_asset_id: String // 44 max
}

impl Background {
    pub const LEN: usize = 8
        + 50;
}