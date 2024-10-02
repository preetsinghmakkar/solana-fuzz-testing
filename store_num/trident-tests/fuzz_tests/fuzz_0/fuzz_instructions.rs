pub mod store_num_fuzz_instructions {
    use crate::accounts_snapshots::*;
    use solana_sdk::native_token::LAMPORTS_PER_SOL;
    use solana_sdk::system_program::ID as SYSTEM_PROGRAM_ID;
    use trident_client::fuzzing::*;
    #[derive(Arbitrary, DisplayIx, FuzzTestExecutor, FuzzDeserialize)]
    pub enum FuzzInstruction {
        Initialize(Initialize),
    }
    #[derive(Arbitrary, Debug)]
    pub struct Initialize {
        pub accounts: InitializeAccounts,
        pub data: InitializeData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitializeAccounts {
        pub signer: AccountId,
        pub store_number_account: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitializeData {
        pub _num: u8,
    }
    impl<'info> IxOps<'info> for Initialize {
        type IxData = store_num::instruction::Initialize;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = InitializeSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = store_num::instruction::Initialize {
                _num: self.data._num,
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            //Declare Signer
            let signer = fuzz_accounts.signer.get_or_create_account(
                self.accounts.signer,
                client,
                5 * LAMPORTS_PER_SOL,
            );

            //Declare Account
            let store_num_account = fuzz_accounts
                .store_number_account
                .get_or_create_account(
                    self.accounts.store_number_account,
                    &[b"Store_Number"],
                    &store_num::ID,
                )
                .unwrap();

            let signers = vec![signer.clone()];
            let acc_meta = store_num::accounts::Initialize {
                signer: signer.pubkey(),
                store_number_account: store_num_account.pubkey(),
                system_program: SYSTEM_PROGRAM_ID,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    #[doc = r" Use AccountsStorage<T> where T can be one of:"]
    #[doc = r" Keypair, PdaStore, TokenStore, MintStore, ProgramStore"]
    #[derive(Default)]
    pub struct FuzzAccounts {
        signer: AccountsStorage<Keypair>,
        store_number_account: AccountsStorage<PdaStore>,
        // system_program: AccountsStorage<todo!()>,
    }
}
