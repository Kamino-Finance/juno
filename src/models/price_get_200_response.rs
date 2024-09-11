/// PriceGet200Response : Default response with ids which return an object. Refer to Price hash model below. If the id is invalid, it will not return in the hash.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PriceGet200Response {
    #[serde(rename = "data")]
    pub data: ::std::collections::HashMap<String, crate::models::SwapPrice>,
    #[serde(rename = "timeTaken")]
    pub time_taken: Option<f32>,
    #[serde(rename = "contextSlot")]
    pub context_slot: Option<f32>,
}
