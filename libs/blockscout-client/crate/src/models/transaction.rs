/*
 * BlockScout API
 *
 * API for BlockScout web app
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: you@your-company.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(derive_new::new, Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(rename = "fee")]
    pub fee: models::Fee,
    #[serde(rename = "gas_limit")]
    pub gas_limit: String, // changed
    #[serde(rename = "block_number")]
    pub block_number: i32,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "method")]
    pub method: Option<String>, // changed
    #[serde(rename = "confirmations")]
    pub confirmations: i32,
    #[serde(rename = "type")]
    pub r#type: i32,
    #[serde(rename = "exchange_rate")]
    pub exchange_rate: String,
    #[serde(rename = "to")]
    pub to: Option<models::AddressParam>, // changed
    #[serde(rename = "transaction_burnt_fee")]
    pub transaction_burnt_fee: Option<String>, // changed
    #[serde(rename = "max_fee_per_gas")]
    pub max_fee_per_gas: Option<String>, // changed
    #[serde(rename = "result")]
    pub result: String,
    #[serde(rename = "hash")]
    pub hash: String,
    #[serde(rename = "gas_price")]
    pub gas_price: String,
    #[serde(rename = "priority_fee")]
    pub priority_fee: Option<String>, // changed
    #[serde(rename = "base_fee_per_gas")]
    pub base_fee_per_gas: String,
    #[serde(rename = "from")]
    pub from: models::AddressParam,
    #[serde(rename = "token_transfers")]
    pub token_transfers: Option<Vec<models::TokenTransfer>>, //changed
    #[serde(rename = "transaction_types")]
    pub transaction_types: Vec<String>,
    #[serde(rename = "gas_used")]
    pub gas_used: String,
    #[serde(rename = "created_contract")]
    pub created_contract: Option<models::AddressParam>, // changed
    #[serde(rename = "position")]
    pub position: i32,
    #[serde(rename = "nonce")]
    pub nonce: i32,
    #[serde(rename = "has_error_in_internal_transactions")]
    pub has_error_in_internal_transactions: Option<bool>, // changed
    #[serde(rename = "actions")]
    pub actions: Vec<models::TransactionAction>,
    #[serde(rename = "decoded_input")]
    pub decoded_input: Option<models::DecodedInput>, // changed
    #[serde(rename = "token_transfers_overflow")]
    pub token_transfers_overflow: Option<bool>, // changed
    #[serde(rename = "raw_input")]
    pub raw_input: String,
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "max_priority_fee_per_gas")]
    pub max_priority_fee_per_gas: Option<String>, // changed
    #[serde(rename = "revert_reason")]
    pub revert_reason: Option<serde_json::Value>, // changed
    #[serde(rename = "confirmation_duration")]
    pub confirmation_duration: serde_json::Value,
    #[serde(rename = "transaction_tag")]
    pub transaction_tag: Option<String>, // changed
}
