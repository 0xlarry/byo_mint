use crate::*;

#[account]
pub struct FaucetV2 {
    pub authority: Pubkey, 
    pub collection_mint: Pubkey,
    pub merkle_tree: Pubkey,
    pub current_supply: u64,
    pub supply_cap: u64,
    pub mint_price: u64,
    pub layer_map: Pubkey,
    pub supply_map: Pubkey,
    pub open_map: Pubkey,
    pub mint_token: Pubkey,
    pub bump: u8,
}


impl FaucetV2 {
    pub const LEN: usize = 8 
        + 32 
        + 32 
        + 32
        + 8
        + 8
        + 8
        + 32
        + 32
        + 32
        + 32
        + 1;

    pub fn new(
        authority: Pubkey, 
        collection_mint: Pubkey,
        merkle_tree: Pubkey,
        supply_cap: u64,
        mint_price: u64,
        layer_map: &Option<Box<Account<LayerMap>>>,
        supply_map: &Option<Box<Account<SupplyMap>>>,
        open_map: &Option<Box<Account<OpenMap>>>,
        mint_token: Pubkey,
        bump: u8,
    ) -> Result<FaucetV2> {
        // ensure faucet auth == md map auths
        let lm = match layer_map {
            Some(x) => {
                require!(x.authority == authority, ByomError::InvalidAuthority);
                x.key()
            },
            None => Pubkey::default()
        };
        let sm = match supply_map {
            Some(x) => {
                require!(x.authority == authority, ByomError::InvalidAuthority);
                x.key()
            },
            None => Pubkey::default()
        };
        let om: Pubkey = match open_map {
            Some(x) => {
                require!(x.authority == authority, ByomError::InvalidAuthority);
                x.key()
            },
            None => Pubkey::default()
        };

        Ok(FaucetV2 {
            authority,
            collection_mint, 
            merkle_tree, 
            current_supply: 0,
            supply_cap,
            mint_price, 
            layer_map: lm,
            supply_map: sm,
            open_map: om,
            mint_token,
            bump
        })
    }

    pub fn mint_requirements(
        &mut self, 
        merkle_tree: Pubkey, 
        creator: Pubkey, 
        layer_map: Option<&mut Account<LayerMap>>, 
        supply_map: Option<&mut Account<SupplyMap>>, 
        open_map: Option<&mut Account<OpenMap>>,
    ) -> Result<()> {
        require!(self.merkle_tree == merkle_tree, ByomError::InvalidAccount);
        require!(self.current_supply < self.supply_cap, ByomError::SupplyCap);
        match layer_map {
            Some(x) => {
                require!(x.key() == self.layer_map, ByomError::InvalidAccount);
                require!(creator == x.creators[0].address, ByomError::InvalidAccount);
            },
            None => {}
        }
        match supply_map {
            Some(x) => {
                require!(x.key() == self.supply_map, ByomError::InvalidAccount);
                require!(creator == x.creators[0].address, ByomError::InvalidAccount);
            },
            None => {}
        }
        match open_map {
            Some(x) => {
                require!(x.key() == self.open_map, ByomError::InvalidAccount);
                require!(creator == x.creators[0].address, ByomError::InvalidAccount);
            },
            None => {}
        }
        Ok(())
    }

    // TODO WRITE GENERIC FEE PAYER
    // pub fn pay_fee(
    //     &mut self,
    //     creator: &mut AccountInfo,
    //     minter: &mut Signer,
    //     minter_ta: &mut Option<Account<TokenAccount>>,
    //     creator_ta: &mut Option<Account<TokenAccount>>,
    //     system_program: &mut Program<System>,
    //     token_program: &mut Program<TokenProgram>
    // ) -> Result<()> {
    //     // pay fees to FIRST CREATOR
    //     if self.mint_token == Pubkey::default() {
    //         invoke(
    //             &system_instruction::transfer(
    //                 &minter.key(), 
    //                 &creator.key(), 
    //                 self.mint_price
    //             ), 
    //         &[
    //             minter.clone().to_account_info(),
    //             creator.to_account_info(),
    //             system_program.to_account_info(),
    //         ])?;
    //     } else {
    //         let mta;
    //         match minter_ta {
    //             Some(ta) => {mta = ta;},
    //             None => {return Err(ByomError::InvalidAccount.into());}
    //         }
    //         let cta;
    //         match creator_ta {
    //             Some(ta) => {cta = ta;},
    //             None => {return Err(ByomError::InvalidAccount.into());}
    //         }
    //         require!(cta.owner == creator.key(), ByomError::InvalidAccount);
    //         require!(
    //             mta.mint == self.mint_token && cta.mint == self.mint_token,
    //             ByomError::InvalidAccount
    //         );
    //         transfer(
    //             CpiContext::new(
    //                 token_program.to_account_info(),
    //                 Transfer {
    //                     from: mta.to_account_info(),
    //                     to: cta.to_account_info(),
    //                     authority: minter.to_account_info(),
    //                 },
    //             ),
    //             self.mint_price,
    //         )?;
    //     }
    //     msg!("PAID FEE");
    //     Ok(())
    // }
}