use serde_with::{serde_as, DisplayFromStr};
use solana_sdk::pubkey::Pubkey;
use std::collections::HashMap;

/// RouteMap : Default response
#[serde_as]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RouteMap {
    /// All the mints that are indexed to match in indexedRouteMap
    #[serde(rename = "mintKeys")]
    #[serde_as(as = "Vec<DisplayFromStr>")]
    pub mint_keys: Vec<Pubkey>,
    /// All the possible route and their corresponding output mints
    #[serde(rename = "indexedRouteMap")]
    #[serde_as(as = "HashMap<DisplayFromStr,Vec<DisplayFromStr>>")]
    pub indexed_route_map: HashMap<usize, Vec<usize>>,
}
