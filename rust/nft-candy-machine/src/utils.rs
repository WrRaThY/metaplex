use anchor_lang::{ProgramAccount};
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction;

use {
    anchor_lang::{
        prelude::{AccountInfo, ProgramError, ProgramResult, Pubkey},
        solana_program::{
            program::invoke_signed,
            program_pack::{IsInitialized, Pack},
        },
    },
    crate::ErrorCode,
};

use crate::{CandyMachine, ConfigData, Config};

pub fn assert_initialized<T: Pack + IsInitialized>(
    account_info: &AccountInfo,
) -> Result<T, ProgramError> {
    let account: T = T::unpack_unchecked(&account_info.data.borrow())?;
    if !account.is_initialized() {
        Err(ErrorCode::Uninitialized.into())
    } else {
        Ok(account)
    }
}

pub fn assert_owned_by(account: &AccountInfo, owner: &Pubkey) -> ProgramResult {
    if account.owner != owner {
        Err(ErrorCode::IncorrectOwner.into())
    } else {
        Ok(())
    }
}

///TokenTransferParams
pub struct TokenTransferParams<'a: 'b, 'b> {
    /// source
    pub source: AccountInfo<'a>,
    /// destination
    pub destination: AccountInfo<'a>,
    /// amount
    pub amount: u64,
    /// authority
    pub authority: AccountInfo<'a>,
    /// authority_signer_seeds
    pub authority_signer_seeds: &'b [&'b [u8]],
    /// token_program
    pub token_program: AccountInfo<'a>,
}

#[inline(always)]
pub fn spl_token_transfer(params: TokenTransferParams<'_, '_>) -> ProgramResult {
    let TokenTransferParams {
        source,
        destination,
        authority,
        token_program,
        amount,
        authority_signer_seeds,
    } = params;

    let result = invoke_signed(
        &spl_token::instruction::transfer(
            token_program.key,
            source.key,
            destination.key,
            authority.key,
            &[],
            amount,
        )?,
        &[source, destination, authority, token_program],
        &[authority_signer_seeds],
    );

    result.map_err(|_| ErrorCode::TokenTransferFailed.into())
}

pub fn send_mint_part<'info>(config: &Config, price: u64, creator_info: AccountInfo<'info>,
                      payer: AccountInfo<'info>, system_program: AccountInfo<'info>) -> ProgramResult {
    let found_creator = config.data.creators.iter()
        .find(|it| it.address.eq(creator_info.key));

    return if let Some(value) = found_creator {
        let lamports = price.checked_mul(value.share as u64).unwrap()
            .checked_div(100).unwrap();

        invoke(
            &system_instruction::transfer(
                payer.key,
                creator_info.key,
                lamports,
            ),
            &[
                payer,
                creator_info,
                system_program,
            ],
        )?;

        Ok(())
    } else {
        Err(ErrorCode::CreatorNotFound.into())
    }

}
