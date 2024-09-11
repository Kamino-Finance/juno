use serde_with::{serde_as, DisplayFromStr};

#[serde_as]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SwapRoute {
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "inputMint")]
    pub input_mint: String,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "inAmount")]
    pub in_amount: u64,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "outputMint")]
    pub output_mint: String,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "outAmount")]
    pub out_amount: u64,
    /// The threshold for the swap based on the provided slippage: when swapMode is ExactIn the minimum out amount, when swapMode is ExactOut the maximum in amount
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "otherAmountThreshold")]
    pub other_amount_threshold: u64,

    #[serde(rename = "swapMode")]
    pub swap_mode: SwapMode,
    #[serde(rename = "slippageBps")]
    pub slippage_bps: i32,
    #[serde(rename = "platformFee")]
    pub fees: Option<i32>,
    #[serde(rename = "priceImpactPct")]
    pub price_impact_pct: String,
    #[serde(rename = "routePlan")]
    pub route_plan: Vec<RoutePlan>,
    #[serde(rename = "contextSlot")]
    pub context_slot: u64,
    #[serde(rename = "timeTaken")]
    pub time_taken: f32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoutePlan {
    pub swap_info: SwapInfo,
    pub percent: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwapInfo {
    pub amm_key: String,
    pub label: String,
    pub input_mint: String,
    pub output_mint: String,
    pub in_amount: String,
    pub out_amount: String,
    pub fee_amount: String,
    pub fee_mint: String,
}

/// Swap mode
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SwapMode {
    ExactIn,
    ExactOut,
}

impl Default for SwapMode {
    fn default() -> SwapMode {
        Self::ExactIn
    }
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Fees {
    /// This inidicate the total amount needed for signing transaction(s). Value in lamports.
    #[serde(rename = "signatureFee")]
    pub signature_fee: Option<f32>,
    /// This inidicate the total amount needed for deposit of serum order account(s). Value in lamports.
    #[serde(rename = "openOrdersDeposits")]
    pub open_orders_deposits: Option<Vec<f32>>,
    /// This inidicate the total amount needed for deposit of associative token account(s). Value in lamports.
    #[serde(rename = "ataDeposits")]
    pub ata_deposits: Option<Vec<f32>>,
    /// This inidicate the total lamports needed for fees and deposits above.
    #[serde(
        rename = "totalFeeAndDeposits",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_fee_and_deposits: Option<f32>,
    /// This inidicate the minimum lamports needed for transaction(s). Might be used to create wrapped SOL and will be returned when the wrapped SOL is closed.
    #[serde(
        rename = "minimumSOLForTransaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub minimum_sol_for_transaction: Option<f32>,
}
