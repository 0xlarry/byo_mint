pub mod create_faucet;
pub use create_faucet::*;
pub mod add_new_tree;
pub use add_new_tree::*;
pub mod mint_cnft;
pub use mint_cnft::*;
pub mod withdraw_fees;
pub use withdraw_fees::*;
pub mod update_faucet;
pub use update_faucet::*;
pub const COLLECTION_CPI_PREFIX: &str = "collection_cpi";
pub mod create_faucet_wl;
pub use create_faucet_wl::*;
pub mod mint_cnft_wl;
pub use mint_cnft_wl::*;
pub mod add_new_tree_wl;
pub use add_new_tree_wl::*;
pub mod update_faucet_wl;
pub use update_faucet_wl::*;
pub mod withdraw_fees_wl;
pub use withdraw_fees_wl::*;