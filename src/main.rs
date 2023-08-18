use axum::{
    http::{Method, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{collections::HashMap, env, net::SocketAddr};
use tower_http::cors::Any;
use tower_http::cors::CorsLayer;

use dotenv::dotenv;
use fuels::accounts::fuel_crypto::SecretKey;
use fuels::{prelude::*, types::ContractId};
use std::str::FromStr;

mod structs;
use structs::*;

const SECRET_KEY: &str = "49c93db298f06769c8c5a4626b2dd8fb06faae8d1a6974f19616338c3a0a7bee";

// Load abi from json
abigen!(Contract(
    name = "Orderbook",
    abi = "../out/debug/orderbook-abi.json"
));

pub async fn root() -> &'static str {
    "ok"
}

pub async fn open_orders(Json(req): Json<OpenOrderReq>) -> impl IntoResponse {
    let open_orders: Vec<OpenLimitOrderVO> = vec![];

    let mut address = "".to_string();
    let market = req.market;
    if market == "ETH-USDC" {
        address = env::var("ETHUSDC").expect("❌ Expected ETHUSDC in the environment");
    } else if market == "BTC-USDC" {
        address = env::var("BTCUSDC").expect("❌ Expected ETHUSDC in the environment");
    }

    tracing::info!("start open_orders {:?}, {:?}", market, address);
    // Create a provider pointing to the testnet.
    let provider = match Provider::connect("beta-3.fuel.network").await {
        Ok(p) => p,
        Err(error) => panic!("❌ Problem creating provider: {:#?}", error),
    };

    // Setup a private key
    let secret = SecretKey::from_str(SECRET_KEY).unwrap();
    // Create the wallet.
    let wallet = WalletUnlocked::new_from_private_key(secret, Some(provider));

    let bech32_id = Bech32ContractId::from(ContractId::from_str(&address).unwrap());
    let instance = Orderbook::new(bech32_id.clone(), wallet.clone());

    let bids = instance.methods().orderbook(0).simulate().await;
    let bid_data: Vec<OpenLimitOrder> = bids.unwrap().value.to_vec();
    tracing::info!("    bids {:?}", bid_data);
    let asks = instance.methods().orderbook(1).simulate().await;
    let ask_data = asks.unwrap().value.to_vec();
    tracing::info!("    asks {:?}", ask_data);

    let mut user_orders: Vec<OpenLimitOrder> = bid_data
        .iter()
        .filter(|&o| o.address.to_string() == req.account)
        .cloned()
        .collect();

    let user_ask_orders: Vec<OpenLimitOrder> = ask_data
        .iter()
        .filter(|&o| o.address.to_string() == req.account)
        .cloned()
        .collect();

    user_orders.extend(user_ask_orders);

    let view: Vec<OpenLimitOrderVO> = user_orders
        .iter()
        .map(|o| OpenLimitOrderVO {
            address: o.address.to_string(),
            amount: o.amount,
            price: o.price,
            seq: o.seq,
            filled: o.filled,
            side: o.side,
            timestamp: o.timestamp,
        })
        .collect();

    (StatusCode::OK, Json(json!({"code":0,"data": view })))
}

pub async fn trades(Json(req): Json<UserTradeReq>) -> impl IntoResponse {
    let trades: Vec<TradeVO> = vec![];

    let market = req.market;
    let mut address = "".to_string();
    if market == "ETH-USDC" {
        address = env::var("ETHUSDC").expect("❌ Expected ETHUSDC in the environment");
    } else if market == "BTC-USDC" {
        address = env::var("BTCUSDC").expect("❌ Expected ETHUSDC in the environment");
    }

    tracing::info!("start get_orderbook {:?}, {:?}", market, address);
    // Create a provider pointing to the testnet.
    let provider = match Provider::connect("beta-3.fuel.network").await {
        Ok(p) => p,
        Err(error) => panic!("❌ Problem creating provider: {:#?}", error),
    };

    // Setup a private key
    let secret = SecretKey::from_str(SECRET_KEY).unwrap();
    // Create the wallet.
    let wallet = WalletUnlocked::new_from_private_key(secret, Some(provider));

    let bech32_id = Bech32ContractId::from(ContractId::from_str(&address).unwrap());
    let instance = Orderbook::new(bech32_id.clone(), wallet.clone());

    let trades = instance.methods().recent_trades(0).simulate().await;
    let user_trades = trades.unwrap().value.to_vec();
    tracing::info!("    trades {:?}", user_trades);

    let view: Vec<TradeVO> = user_trades
        .iter()
        .map(|o| TradeVO {
            maker: o.maker.to_string(),
            taker: o.taker.to_string(),
            price: o.price,
            amount: o.amount,
            side: o.side,
            timestamp: o.timestamp,
        })
        .collect();

    (StatusCode::OK, Json(json!({"code":0,"data": view })))
}

async fn trade_history() -> Vec<OpenLimitOrder> {
    let open_orders: Vec<OpenLimitOrder> = vec![];
    open_orders
}

async fn get_orderbook(market: String) -> OrderbookView {
    let mut address = "".to_string();
    if market == "ETH-USDC" {
        address = env::var("ETHUSDC").expect("❌ Expected ETHUSDC in the environment");
    } else if market == "BTC-USDC" {
        address = env::var("BTCUSDC").expect("❌ Expected ETHUSDC in the environment");
    }

    tracing::info!("start get_orderbook {:?}, {:?}", market, address);
    // Create a provider pointing to the testnet.
    let provider = match Provider::connect("beta-3.fuel.network").await {
        Ok(p) => p,
        Err(error) => panic!("❌ Problem creating provider: {:#?}", error),
    };

    // Setup a private key
    let secret = SecretKey::from_str(SECRET_KEY).unwrap();
    // Create the wallet.
    let wallet = WalletUnlocked::new_from_private_key(secret, Some(provider));

    let bech32_id = Bech32ContractId::from(ContractId::from_str(&address).unwrap());
    let instance = Orderbook::new(bech32_id.clone(), wallet.clone());

    let bids = instance.methods().orderbook(0).simulate().await;
    let bid_data: Vec<OpenLimitOrder> = bids.unwrap().value.to_vec();
    tracing::info!("    bids {:?}", bid_data);
    let asks = instance.methods().orderbook(1).simulate().await;
    let ask_data = asks.unwrap().value.to_vec();
    tracing::info!("    asks {:?}", ask_data);

    let mut bids_map: HashMap<u64, u64> = HashMap::new();

    bid_data.into_iter().for_each(|v: OpenLimitOrder| {
        // println!("{:?}", v);
        let price = v.price;
        match bids_map.get(&price) {
            Some(val) => {
                bids_map.insert(price, *val + v.amount);
            }
            None => {
                bids_map.insert(price, v.amount);
            }
        };
    });

    let mut asks_map: HashMap<u64, u64> = HashMap::new();
    ask_data.into_iter().for_each(|v: OpenLimitOrder| {
        // println!("{:?}", v);
        let price = v.price;
        match asks_map.get(&price) {
            Some(val) => {
                asks_map.insert(price, *val + v.amount);
            }
            None => {
                asks_map.insert(price, v.amount);
            }
        };
    });

    let mut vec_ask = Vec::from_iter(asks_map.clone());
    let mut vec_bid: Vec<(u64, u64)> = Vec::from_iter(bids_map.clone());
    vec_ask.sort_by(|a, b| a.0.cmp(&b.0));
    vec_bid.sort_by(|a, b| b.0.cmp(&a.0));
    tracing::info!("    vec_ask :{:?} ", vec_ask);
    tracing::info!("    vec_bid :{:?} ", vec_bid);
    tracing::info!("end get_orderbook ");
    OrderbookView {
        bids: vec_bid,
        asks: vec_ask,
    }
}

pub async fn orderbook(Json(req): Json<OrderbookReq>) -> impl IntoResponse {
    let orderbook: OrderbookView = get_orderbook(req.market).await;
    (StatusCode::OK, Json(json!({"code":0,"data": orderbook })))
}

// pub async fn kline(Json(req): Json<KineReq>) -> impl IntoResponse {
//     let client = reqwest::Client::new();
//     let url = format!(
//         "https://api.binance.com/api/v3/klines?symbol={}&interval=4h",
//         "ETHUSDC"
//     );
//     tracing::info!("kline url {:?}", url);
//     let res = client.get(url).send().await;
//     if res.is_err() {
//         tracing::error!("get kline error {:?}", res);
//         return (StatusCode::OK, Json(json!({"code":0,"data": "" })));
//     }
//     let kline = res.unwrap().text().await.unwrap();
//     tracing::info!("kline: {:?}", kline);

//     (StatusCode::OK, Json(json!({"code":0,"data": kline })))
// }

#[tokio::main]
async fn main() {
    dotenv().ok();
    let eth_usdc = env::var("ETHUSDC").expect("❌ Expected ETHUSDC in the environment");
    let btc_usdc = env::var("BTCUSDC").expect("❌ Expected BTCUSDC in the environment");
    let str_port = env::var("PORT").expect("❌ Expected PORT in the environment");

    tracing_subscriber::fmt::init();
    tracing::info!("ETHUSDC {}", eth_usdc);
    tracing::info!("BTCUSDC {}", btc_usdc);

    let cors = CorsLayer::new()
        .allow_headers(Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_origin(Any);

    let app = Router::new()
        .route("/", get(root))
        .route("/orderbook", post(orderbook))
        // .route("/user_trades", post(trades))
        .route("/trades", post(trades))
        .route("/open_order", post(open_orders))
        // .route("/kline", post(kline))
        .layer(cors);

    let port: u16 = str_port.parse().unwrap();
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
