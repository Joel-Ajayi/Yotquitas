use crate::node::Node;
use anyhow::Result;
use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use yotquitas_core::{Address, Transaction};

/// JSON-RPC request
#[derive(Debug, Deserialize)]
pub struct JsonRpcRequest {
    jsonrpc: String,
    method: String,
    params: serde_json::Value,
    id: serde_json::Value,
}

/// JSON-RPC response
#[derive(Debug, Serialize)]
pub struct JsonRpcResponse {
    jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<JsonRpcError>,
    id: serde_json::Value,
}

/// JSON-RPC error
#[derive(Debug, Serialize)]
pub struct JsonRpcError {
    code: i32,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<serde_json::Value>,
}

/// Create the API router
pub fn create_router(node: Arc<Node>) -> Router {
    Router::new()
        .route("/", post(handle_jsonrpc))
        .with_state(node)
}

/// Handle JSON-RPC requests
async fn handle_jsonrpc(
    State(node): State<Arc<Node>>,
    Json(request): Json<JsonRpcRequest>,
) -> Result<Json<JsonRpcResponse>, StatusCode> {
    let response = match request.method.as_str() {
        "eth_sendRawTransaction" => {
            handle_send_raw_transaction(node, request.params, request.id).await
        }
        "eth_getBlockByNumber" => {
            handle_get_block_by_number(node, request.params, request.id).await
        }
        "aequitas_getAccountBalance" => {
            handle_get_account_balance(node, request.params, request.id).await
        }
        _ => Ok(JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            result: None,
            error: Some(JsonRpcError {
                code: -32601,
                message: "Method not found".to_string(),
                data: None,
            }),
            id: request.id,
        }),
    };

    response.map(|r| Json(r)).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

/// Handle eth_sendRawTransaction
async fn handle_send_raw_transaction(
    node: Arc<Node>,
    params: serde_json::Value,
    id: serde_json::Value,
) -> Result<JsonRpcResponse> {
    let tx_hex = params
        .as_array()
        .and_then(|arr| arr.get(0))
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Invalid params"))?;

    // Decode hex transaction
    let tx_bytes = hex::decode(tx_hex.trim_start_matches("0x"))
        .map_err(|_| anyhow::anyhow!("Invalid hex"))?;

    let tx: Transaction = serde_json::from_slice(&tx_bytes)
        .map_err(|_| anyhow::anyhow!("Invalid transaction"))?;

    // Process transaction
    match node.process_transaction(tx.clone()).await {
        Ok(_) => {
            let tx_hash = hex::encode(tx.hash());
            Ok(JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: Some(serde_json::json!(format!("0x{}", tx_hash))),
                error: None,
                id,
            })
        }
        Err(e) => Ok(JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            result: None,
            error: Some(JsonRpcError {
                code: -32000,
                message: e.to_string(),
                data: None,
            }),
            id,
        }),
    }
}

/// Handle eth_getBlockByNumber
async fn handle_get_block_by_number(
    node: Arc<Node>,
    params: serde_json::Value,
    id: serde_json::Value,
) -> Result<JsonRpcResponse> {
    let block_num = params
        .as_array()
        .and_then(|arr| arr.get(0))
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Invalid params"))?;

    // For MVP, only support "latest"
    if block_num == "latest" || block_num == "\"latest\"" {
        match node.get_latest_block().await {
            Ok(Some(block)) => {
                let block_json = serde_json::to_value(&block)
                    .map_err(|_| anyhow::anyhow!("Serialization error"))?;
                Ok(JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    result: Some(block_json),
                    error: None,
                    id,
                })
            }
            Ok(None) => Ok(JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: Some(serde_json::Value::Null),
                error: None,
                id,
            }),
            Err(e) => Ok(JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: None,
                error: Some(JsonRpcError {
                    code: -32000,
                    message: e.to_string(),
                    data: None,
                }),
                id,
            }),
        }
    } else {
        Ok(JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            result: None,
            error: Some(JsonRpcError {
                code: -32602,
                message: "Invalid params: only 'latest' is supported".to_string(),
                data: None,
            }),
            id,
        })
    }
}

/// Handle aequitas_getAccountBalance
async fn handle_get_account_balance(
    node: Arc<Node>,
    params: serde_json::Value,
    id: serde_json::Value,
) -> Result<JsonRpcResponse> {
    let address_hex = params
        .as_array()
        .and_then(|arr| arr.get(0))
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Invalid params"))?;

    // Decode address
    let address_bytes = hex::decode(address_hex.trim_start_matches("0x"))
        .map_err(|_| anyhow::anyhow!("Invalid hex"))?;

    if address_bytes.len() != 32 {
        return Ok(JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            result: None,
            error: Some(JsonRpcError {
                code: -32602,
                message: "Invalid address length".to_string(),
                data: None,
            }),
            id,
        });
    }

    let address: Address = address_bytes
        .try_into()
        .map_err(|_| anyhow::anyhow!("Invalid address"))?;

    match node.get_balance(&address) {
        Ok(balance) => Ok(JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            result: Some(serde_json::json!(format!("0x{:x}", balance))),
            error: None,
            id,
        }),
        Err(e) => Ok(JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            result: None,
            error: Some(JsonRpcError {
                code: -32000,
                message: e.to_string(),
                data: None,
            }),
            id,
        }),
    }
}
