/// QuoteGet200Response : Default response
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct QuoteGet200Response {
    pub data: crate::models::SwapRoute,
    #[serde(rename = "timeTaken")]
    pub time_taken: Option<f32>,
    #[serde(rename = "contextSlot")]
    pub context_slot: Option<i32>,
}
