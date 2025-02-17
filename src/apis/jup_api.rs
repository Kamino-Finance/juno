/*
 * Jupiter APIv4
 *
 * Jupiter quote and swap API
 *
 * The version of the OpenAPI document: 0.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

use reqwest;
use solana_sdk::pubkey::Pubkey;

use super::{configuration, Error};
use crate::{apis::ResponseContent, SwapRoute};

/// struct for typed errors of method [`indexed_route_map_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IndexedRouteMapGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`price_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PriceGetError {
    Status400(serde_json::Value),
    Status404(serde_json::Value),
    Status409(crate::models::PriceGet409Response),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`quote_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QuoteGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`swap_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SwapPostError {
    UnknownValue(serde_json::Value),
}

/// Returns a hash map, input mint as key and an array of valid output mint as values, token mints are indexed to reduce the file size
pub async fn indexed_route_map_get(
    configuration: &configuration::Configuration,
    only_direct_routes: bool,
) -> Result<crate::models::RouteMap, Error<IndexedRouteMapGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v4/indexed-route-map", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if only_direct_routes {
        local_var_req_builder = local_var_req_builder.query(&[("onlyDirectRoutes", "true")]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<IndexedRouteMapGetError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get simple price for a given input mint, output mint and amount
pub async fn prices_get(
    configuration: &configuration::Configuration,
    ids: &[Pubkey],
    vs_token: &Pubkey,
    vs_amount: f32,
) -> Result<crate::models::PriceGet200Response, Error<PriceGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = "https://quote-api.jup.ag/v4/price";
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str);

    let merged_ids: String = ids
        .iter()
        .map(ToString::to_string)
        .reduce(|ids, id| ids + "," + &id)
        .unwrap();
    local_var_req_builder = local_var_req_builder.query(&[("ids", &merged_ids)]);
    local_var_req_builder = local_var_req_builder.query(&[("vsToken", vs_token.to_string())]);

    local_var_req_builder = local_var_req_builder.query(&[("vsAmount", vs_amount.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PriceGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

#[allow(clippy::too_many_arguments)]
/// Get quote for a given input mint, output mint and amount
pub async fn quote_get(
    configuration: &configuration::Configuration,
    input_mint: &Pubkey,
    output_mint: &Pubkey,
    amount: u64,
    slippage_bps: Option<u16>,
    only_direct_routes: bool,
    as_legacy_transaction: bool,
    max_accounts: Option<u8>,
) -> Result<SwapRoute, Error<QuoteGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/quote", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("inputMint", &input_mint.to_string())]);
    local_var_req_builder =
        local_var_req_builder.query(&[("outputMint", &output_mint.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("amount", &amount.to_string())]);
    if let Some(ref local_var_str) = slippage_bps {
        local_var_req_builder =
            local_var_req_builder.query(&[("slippageBps", &local_var_str.to_string())]);
    }
    if only_direct_routes {
        local_var_req_builder = local_var_req_builder.query(&[("onlyDirectRoutes", "true")]);
    }
    if as_legacy_transaction {
        local_var_req_builder = local_var_req_builder.query(&[("asLegacyTransaction", "true")]);
    }
    if let Some(ref local_var_str) = max_accounts {
        local_var_req_builder =
            local_var_req_builder.query(&[("maxAccounts", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<QuoteGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get swap serialized transactions for a route
pub async fn swap_post(
    configuration: &configuration::Configuration,
    body: crate::models::SwapPostRequest,
) -> Result<crate::models::SwapPost200Response, Error<SwapPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/swap", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SwapPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
