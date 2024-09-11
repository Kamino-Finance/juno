/// PriceGet409Response : Duplicate symbol found for input or vsToken. The server will respond an error structure which contains the conflict addresses. User will have to use address mode instead.
#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct PriceGet409Response {
    #[serde(rename = "data")]
    pub data: Option<PriceGet409ResponseData>,
}

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct PriceGet409ResponseData {
    /// Duplicated symbol found, use one of the address instead
    #[serde(rename = "error")]
    pub error: Option<String>,
    /// List of addresses for the symbol
    #[serde(rename = "addresses")]
    pub addresses: Option<Vec<String>>,
}
