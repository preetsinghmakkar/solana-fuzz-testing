use anchor_lang::prelude::*;
use trident_client::fuzzing::{anchor_lang, FuzzingError};
pub struct InitializeSnapshot<'info> {
    pub signer: Signer<'info>,
    pub store_number_account: Option<Account<'info, store_num::StoreNumber>>,
    pub system_program: Program<'info, System>,
}
impl<'info> InitializeSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let signer: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("signer".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("signer".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("signer".to_string()))?;
        let store_number_account: Option<
            anchor_lang::accounts::account::Account<store_num::StoreNumber>,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "store_number_account".to_string(),
            ))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    anchor_lang::accounts::account::Account::try_from(acc).map_err(|_| {
                        FuzzingError::CannotDeserializeAccount("store_number_account".to_string())
                    })
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "store_number_account".to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        let system_program: anchor_lang::accounts::program::Program<System> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "system_program".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("system_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("system_program".to_string()))?;
        Ok(Self {
            signer,
            store_number_account,
            system_program,
        })
    }
}
