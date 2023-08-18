use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct OpenOrderReq {
    pub market: String,
    pub account: String,
}

#[derive(Debug, Deserialize)]
pub struct UserTradeReq {
    pub market: String,
    pub account: String,
}

#[derive(Debug, Deserialize)]
pub struct OrderbookReq {
    pub market: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenLimitOrderVO {
    pub address: String,
    pub price: u64,
    pub amount: u64,
    pub seq: u64,
    pub filled: u64,
    pub side: u64,
    pub timestamp: u64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TradeVO {
    pub maker: String,
    pub taker: String,
    pub price: u64,
    pub amount: u64,
    pub timestamp: u64,
    pub side: u64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderbookView {
    pub bids: Vec<(u64, u64)>,
    pub asks: Vec<(u64, u64)>,
}
impl OrderbookView {
    pub fn new() -> Self {
        OrderbookView {
            bids: vec![],
            asks: vec![],
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct KineReq {
    pub market: String,
}

extern crate serde_json;

pub type Kline = Vec<Vec<KlineElement>>;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum KlineElement {
    Integer(i64),

    String(String),
}
