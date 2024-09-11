use serde_with::{serde_as, skip_serializing_none, DisplayFromStr};
use solana_sdk::pubkey::Pubkey;

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SwapPrice {
    /// Address of the token
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "id")]
    pub id: Pubkey,
    /// Symbol of the token
    #[serde(rename = "mintSymbol")]
    pub mint_symbol: Option<String>,
    /// Address of the vs token
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "vsToken")]
    pub vs_token: Pubkey,
    /// Symbol of the vs token
    #[serde(rename = "vsTokenSymbol")]
    pub vs_token_symbol: Option<String>,
    /// Default to 1 unit of the token worth in USDC if vsToken is not specified.
    #[serde(rename = "price")]
    pub price: f32,
}
