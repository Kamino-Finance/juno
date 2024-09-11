#[macro_use]
extern crate serde_derive;

extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;

mod apis;
pub mod errors;
mod models;
pub mod utils;
use std::collections::HashMap;

pub use apis::configuration::DEFAULT_BASE_URL;
use solana_sdk::transaction::VersionedTransaction;
use utils::decompile_transaction_instructions_with_async_fetcher;

use crate::apis::{configuration::Configuration, jup_api};
use crate::models::SwapPostRequest;

use reexports::*;

pub mod reexports {
    pub use solana_sdk::pubkey::Pubkey;
}

pub use errors::{Error, Result};
pub use models::{swap_price::SwapPrice, swap_route::SwapMode, RouteMap, SwapRoute};
pub use utils::AsyncAccountFetcher;
pub use utils::DecompiledVersionedTx;

use std::sync::OnceLock;

static BASE_URL: OnceLock<String> = OnceLock::new();

pub fn set_base_url(url: String) -> Result<()> {
    BASE_URL.set(url).map_err(|_| Error::BaseUrlAlreadySet)
}

pub fn get_base_url() -> &'static str {
    BASE_URL.get_or_init(|| DEFAULT_BASE_URL.to_string())
}

/// Get simple price for a given input mint, output mint and amount
pub async fn get_prices(
    input_mints: &[Pubkey],
    output_mint: &Pubkey,
    amount: f32,
) -> Result<HashMap<String, SwapPrice>> {
    let raw_price = jup_api::prices_get(
        &Configuration::new(get_base_url()),
        input_mints,
        output_mint,
        amount,
    )
    .await?;
    Ok(raw_price.data)
}

#[allow(clippy::too_many_arguments)]
/// Get quotes for a given input mint, output mint and amount
pub async fn get_quote(
    input_mint: &Pubkey,
    output_mint: &Pubkey,
    amount: u64,
    only_direct_routes: bool,
    slippage_bps: Option<u16>,
    max_accounts: Option<u8>,
) -> Result<SwapRoute> {
    let raw_quote = jup_api::quote_get(
        &Configuration::new(get_base_url()),
        input_mint,
        output_mint,
        amount,
        slippage_bps,
        only_direct_routes,
        false,
        max_accounts,
    )
    .await?;
    Ok(raw_quote)
}

/// Get swap serialized transactions for a quote
pub async fn get_swap_transactions(
    route: impl Into<Box<SwapRoute>>,
    user_public_key: Pubkey,
) -> Result<VersionedTransaction> {
    let request = SwapPostRequest {
        route: route.into(),
        user_public_key,
        wrap_unwrap_sol: false,
        fee_account: None,
        destination_wallet: None,
        as_legacy_transaction: false,
    };
    let raw_swap = jup_api::swap_post(&Configuration::new(get_base_url()), request).await?;

    let decode = |base64_transaction: String| -> Result<VersionedTransaction> {
        bincode::deserialize(&base64::decode(base64_transaction)?).map_err(|err| err.into())
    };

    decode(raw_swap.swap_transaction)
}

/// Returns a hash map, input mint as key and an array of valid output mint as values
pub async fn get_route_map(only_direct_routes: bool) -> Result<RouteMap> {
    let raw_route_map =
        jup_api::indexed_route_map_get(&Configuration::new(get_base_url()), only_direct_routes)
            .await?;
    Ok(raw_route_map)
}

/// Get decompiled instructions but requires a fetcher to retrieve the lookup tables
///
/// Note: it is very recommended to enforce single swap tx
pub async fn get_swap_instructions(
    route: impl Into<Box<SwapRoute>>,
    user_public_key: Pubkey,
    accounts_fetcher: &impl AsyncAccountFetcher,
) -> Result<DecompiledVersionedTx> {
    let transactions = get_swap_transactions(route, user_public_key).await?;
    decompile_transaction_instructions_with_async_fetcher(transactions, accounts_fetcher).await
}

#[allow(clippy::too_many_arguments)]
/// Get the swap instructions for the best route matching parameters
pub async fn get_best_swap_instructions(
    input_mint: &Pubkey,
    output_mint: &Pubkey,
    amount: u64,
    only_direct_routes: bool,
    slippage_bps: Option<u16>,
    price_impact_limit: Option<f32>,
    max_accounts: Option<u8>,
    user_public_key: Pubkey,
    accounts_fetcher: &impl AsyncAccountFetcher,
) -> Result<DecompiledVersionedTx> {
    let best_route = get_quote(
        input_mint,
        output_mint,
        amount,
        only_direct_routes,
        slippage_bps,
        max_accounts,
    )
    .await?;

    use crate::Error::ResponseTypeConversionError;
    let route_price_impact_pct: f32 = best_route
        .price_impact_pct
        .parse::<f32>()
        .map_err(|_| ResponseTypeConversionError)?;
    if let Some(price_impact_limit) = price_impact_limit {
        if route_price_impact_pct > price_impact_limit {
            return Err(Error::PriceImpactTooHigh(route_price_impact_pct));
        }
    }
    get_swap_instructions(best_route, user_public_key, accounts_fetcher).await
}
