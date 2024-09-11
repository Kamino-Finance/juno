use serde_with::{serde_as, skip_serializing_none, DisplayFromStr};
use solana_sdk::pubkey::Pubkey;

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SwapPostRequest {
    #[serde(rename = "quoteResponse")]
    pub route: Box<crate::models::SwapRoute>,
    /// Public key of the user
    #[serde(rename = "userPublicKey")]
    #[serde_as(as = "DisplayFromStr")]
    pub user_public_key: Pubkey,
    /// Wrap/unwrap SOL
    #[serde(rename = "wrapAndUnwrapSol")]
    pub wrap_unwrap_sol: bool,
    /// Fee token account for the output token (only pass in if you set a feeBps)
    #[serde(rename = "feeAccount")]
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub fee_account: Option<Pubkey>,
    /// Request a legacy transaction rather than the default versioned transaction, needs to be paired with a quote using asLegacyTransaction otherwise the transaction might be too large
    #[serde(rename = "asLegacyTransaction")]
    pub as_legacy_transaction: bool,
    /// Public key of the wallet that will receive the output of the swap, this assumes the associated token account exists, currently adds a token transfer
    #[serde(rename = "destinationWallet")]
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub destination_wallet: Option<Pubkey>,
}
