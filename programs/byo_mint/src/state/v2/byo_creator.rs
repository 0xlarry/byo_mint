use crate::*;

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ByoCreator {
    pub address: Pubkey,
    pub share: u8             
}
impl ByoCreator {
    pub const LEN: usize = 8 
        + 32
        + 1;
}