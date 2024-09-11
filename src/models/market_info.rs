#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MarketInfo {
    pub id: Option<String>,
    pub label: String,
    #[serde(rename = "inputMint")]
    pub input_mint: Option<String>,
    #[serde(rename = "outputMint")]
    pub output_mint: Option<String>,
    #[serde(rename = "notEnoughLiquidity")]
    pub not_enough_liquidity: Option<bool>,
    #[serde(rename = "inAmount")]
    pub in_amount: Option<String>,
    #[serde(rename = "outAmount")]
    pub out_amount: Option<String>,
    #[serde(rename = "minInAmount")]
    pub min_in_amount: Option<String>,
    #[serde(rename = "minOutAmount")]
    pub min_out_amount: Option<String>,
    #[serde(rename = "priceImpactPct")]
    pub price_impact_pct: Option<f32>,
    #[serde(rename = "lpFee")]
    pub lp_fee: Option<LpFee>,
    #[serde(rename = "platformFee")]
    pub platform_fee: Option<LpFee>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LpFee {
    #[serde(rename = "amount")]
    pub amount: Option<String>,
    #[serde(rename = "mint")]
    pub mint: Option<String>,
    #[serde(rename = "pct")]
    pub pct: Option<f32>,
}
