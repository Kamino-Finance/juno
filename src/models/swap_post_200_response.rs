use serde_with::skip_serializing_none;

/// SwapPost200Response : Default response
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct SwapPost200Response {
    /// Base64 encoded transaction
    #[serde(rename = "swapTransaction")]
    pub swap_transaction: String,
}
