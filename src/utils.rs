use async_trait::async_trait;
use itertools::Itertools;
use solana_address_lookup_table_program::state::AddressLookupTable;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_program::hash::Hash;
use solana_program::instruction::AccountMeta;
use solana_program::message::v0::{self, LoadedAddresses, MessageAddressTableLookup};
use solana_program::message::{
    AddressLoader, AddressLoaderError, SanitizedVersionedMessage, SimpleAddressLoader,
    VersionedMessage,
};
use solana_program::pubkey::Pubkey;
use solana_sdk::account::Account;
use solana_sdk::message::SanitizedMessage;
use solana_sdk::signer::Signer;
use solana_sdk::{
    address_lookup_table_account::AddressLookupTableAccount,
    instruction::{Instruction, InstructionError},
    transaction::VersionedTransaction,
};
use std::borrow::Cow;
use std::collections::HashMap;

use crate::errors::{Error, Result};

#[async_trait]
pub trait AsyncAccountFetcher {
    async fn fech_accounts(&self, pubkeys: &[Pubkey]) -> Result<Vec<Option<Account>>>;
    async fn get_latest_blockhash(&self) -> Result<Hash>;
}

#[async_trait]
impl AsyncAccountFetcher for RpcClient {
    async fn fech_accounts(&self, pubkeys: &[Pubkey]) -> Result<Vec<Option<Account>>> {
        self.get_multiple_accounts(pubkeys)
            .await
            .map_err(Into::into)
    }

    async fn get_latest_blockhash(&self) -> Result<Hash> {
        self.get_latest_blockhash().await.map_err(Into::into)
    }
}

/// Return an Optional array with the pubkeys of the lookup tables that need to be fetched to
/// decompile the transaction
pub fn get_lookup_tables_pubkeys(tx: &VersionedTransaction) -> Option<Vec<&Pubkey>> {
    tx.message.address_table_lookups().map(|msg_lookup_array| {
        msg_lookup_array
            .iter()
            .map(|msg_lookup| &msg_lookup.account_key)
            .collect::<Vec<_>>()
    })
}

/// Decompiles a transaction into a `Vec` of `Instruction`
///
/// ## Args
///
/// - `tx`: The transaction to decompile
/// - `address_loader`: Address loader can be built from pre-loaded lookup tables referenced
///     by the transaction. Good option here is a `BasicAddressLoader`
///     Mandatory if the transaction reference a lookup table.
///     None for legacy tx. The lookup tables are expected
pub fn decompile_transaction_instructions(
    tx: VersionedTransaction,
    address_loader: Option<impl AddressLoader>,
) -> Result<Vec<Instruction>> {
    let sanitized_versioned_msg = SanitizedVersionedMessage::try_new(tx.message)?;
    let sanitized_msg = if let Some(address_loader) = address_loader {
        SanitizedMessage::try_new(sanitized_versioned_msg, address_loader)
    } else {
        SanitizedMessage::try_new(sanitized_versioned_msg, SimpleAddressLoader::Disabled)
    }?;
    let mut instructions = Vec::with_capacity(sanitized_msg.instructions().len());
    let account_keys = sanitized_msg.account_keys();
    instructions.extend(sanitized_msg.instructions().iter().map(|compiled_ix| {
        Instruction::new_with_bytes(
            *account_keys
                .get(compiled_ix.program_id_index.into())
                .ok_or(InstructionError::MissingAccount)
                .unwrap(),
            &compiled_ix.data,
            compiled_ix
                .accounts
                .iter()
                .map(|account_index| {
                    let account_index = *account_index as usize;
                    Ok(AccountMeta {
                        is_signer: sanitized_msg.is_signer(account_index),
                        is_writable: sanitized_msg.is_writable(account_index),
                        pubkey: *account_keys
                            .get(account_index)
                            .ok_or(InstructionError::MissingAccount)?,
                    })
                })
                .collect::<std::result::Result<Vec<AccountMeta>, InstructionError>>()
                .unwrap(),
        )
    }));
    Ok(instructions)
}

#[derive(Debug)]
pub struct DecompiledVersionedTx {
    pub lookup_tables: Option<Vec<AddressLookupTableAccount>>,
    pub instructions: Vec<Instruction>,
}

pub async fn decompile_transaction_instructions_with_async_fetcher(
    tx: VersionedTransaction,
    accounts_fetcher: &impl AsyncAccountFetcher,
) -> Result<DecompiledVersionedTx> {
    let lookup_tables_pk = tx
        .message
        .address_table_lookups()
        .map(|tables| tables.iter().map(|table| table.account_key).collect_vec());
    let address_loader = match lookup_tables_pk {
        Some(ref tables) => Some(
            BasicAddressLoader::from_accounts_and_async_accounts_fetcher(tables, accounts_fetcher)
                .await?,
        ),
        None => None,
    };
    let address_loader_ref = address_loader.as_ref();

    let instructions = decompile_transaction_instructions(tx, address_loader_ref)?;
    Ok(DecompiledVersionedTx {
        lookup_tables: address_loader.map(|l| l.to_address_lookup_table_accounts()),
        instructions,
    })
}

// Note: Needed because Solana only implement `AddressLoader` on banks with all the safety checks
// that we don't need to just decompile a tx.
/// Simple address loader that can be built easily from the fetched/Deserialized `AddressLookupTable`
#[derive(Clone)]
pub struct BasicAddressLoader<'a>(HashMap<Pubkey, AddressLookupTable<'a>>);

impl<'a> BasicAddressLoader<'a> {
    pub fn from_loaded_accounts(
        accounts: &'a [(Pubkey, Account)],
    ) -> Result<BasicAddressLoader<'a>> {
        let mut res = HashMap::with_capacity(accounts.len());
        for (key, account) in accounts {
            let table = AddressLookupTable::deserialize(&account.data)?;
            let _ = res.insert(*key, table);
        }
        Ok(BasicAddressLoader(res))
    }

    pub fn from_accounts_and_accounts_fetcher(
        accounts_pk: &[Pubkey],
        client_closure: impl FnOnce(&[Pubkey]) -> Result<Vec<Option<Account>>>,
    ) -> Result<BasicAddressLoader<'static>> {
        let accounts = client_closure(accounts_pk)?;
        let accounts = accounts
            .into_iter()
            .map(|op_account| op_account.ok_or(Error::LookupTableAccountNotFound))
            .collect::<Result<Vec<_>>>()?;
        let mut res = HashMap::with_capacity(accounts.len());
        for (key, account) in accounts_pk.iter().zip(accounts.into_iter()) {
            let AddressLookupTable { meta, addresses } =
                AddressLookupTable::deserialize(&account.data)?;
            let addresses = Cow::Owned(addresses.into_owned());
            let table = AddressLookupTable { meta, addresses };
            let _ = res.insert(*key, table);
        }
        Ok(BasicAddressLoader(res))
    }

    pub async fn from_accounts_and_async_accounts_fetcher(
        accounts_pk: &[Pubkey],
        accounts_fetcher: &impl AsyncAccountFetcher,
    ) -> Result<BasicAddressLoader<'static>> {
        let accounts = accounts_fetcher.fech_accounts(accounts_pk).await?;
        let accounts = accounts
            .into_iter()
            .map(|op_account| op_account.ok_or(Error::LookupTableAccountNotFound))
            .collect::<Result<Vec<_>>>()?;
        let mut res = HashMap::with_capacity(accounts.len());
        for (key, account) in accounts_pk.iter().zip(accounts.into_iter()) {
            let AddressLookupTable { meta, addresses } =
                AddressLookupTable::deserialize(&account.data)?;
            let addresses = Cow::Owned(addresses.into_owned());
            let table = AddressLookupTable { meta, addresses };
            let _ = res.insert(*key, table);
        }
        Ok(BasicAddressLoader(res))
    }
}
impl BasicAddressLoader<'_> {
    pub fn to_address_lookup_table_accounts(&self) -> Vec<AddressLookupTableAccount> {
        self.0
            .iter()
            .map(|(key, table)| AddressLookupTableAccount {
                key: *key,
                addresses: table.addresses.to_vec(),
            })
            .collect()
    }
}

impl AddressLoader for &BasicAddressLoader<'_> {
    fn load_addresses(
        self,
        lookups: &[MessageAddressTableLookup],
    ) -> std::result::Result<LoadedAddresses, AddressLoaderError> {
        let tables = &self.0;
        // Fill writable
        let writable = lookups
            .iter()
            .map(|lookup| {
                let table = tables
                    .get(&lookup.account_key)
                    .ok_or(AddressLoaderError::LookupTableAccountNotFound)?;
                Ok::<_, AddressLoaderError>(lookup.writable_indexes.iter().map(|index| {
                    table
                        .addresses
                        .get(usize::from(*index))
                        .map(Pubkey::to_owned)
                        .ok_or(AddressLoaderError::InvalidLookupIndex)
                }))
            })
            .flatten_ok()
            .flatten()
            .collect::<std::result::Result<Vec<Pubkey>, AddressLoaderError>>()?;

        // Fill readonly
        let readonly = lookups
            .iter()
            .map(|lookup| {
                let table = tables
                    .get(&lookup.account_key)
                    .ok_or(AddressLoaderError::LookupTableAccountNotFound)?;
                Ok::<_, AddressLoaderError>(lookup.readonly_indexes.iter().map(|index| {
                    table
                        .addresses
                        .get(usize::from(*index))
                        .map(Pubkey::to_owned)
                        .ok_or(AddressLoaderError::InvalidLookupIndex)
                }))
            })
            .flatten_ok()
            .flatten()
            .collect::<std::result::Result<Vec<Pubkey>, AddressLoaderError>>()?;

        Ok(LoadedAddresses { writable, readonly })
    }
}

pub fn create_tx_with_address_table_lookup(
    instructions: &[Instruction],
    address_lookup_tables: &[AddressLookupTableAccount],
    recent_blockhash: Hash,
    payer: &impl Signer,
) -> Result<VersionedTransaction> {
    let tx = VersionedTransaction::try_new(
        VersionedMessage::V0(
            v0::Message::try_compile(
                &payer.pubkey(),
                instructions,
                address_lookup_tables,
                recent_blockhash,
            )
            .map_err(|_| Error::SolanaCompileError)?,
        ),
        &[payer],
    )
    .map_err(|_| Error::SolanaCompileError)?;

    Ok(tx)
}
