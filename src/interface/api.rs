use tokio::net::TcpListener;
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};
use std::net::SocketAddr;

use axum::{
    Router,
    Json,
    routing::{get, post},
    extract::{State, Path},
    http::StatusCode,
    response::IntoResponse,
};

use crate::chain::Blockchain;
use crate::reward::block_reward;

const COINBASE_MATURITY: u64 = 100;

#[derive(Clone)]
struct AppState {
    chain: Arc<Mutex<Blockchain>>,
}

pub async fn start_api(chain: Arc<Mutex<Blockchain>>, port: u16) {
    let state = AppState { chain };

    let app = Router::new()
        .route("/status", get(status))
        .route("/blocks", get(blocks))
        .route("/block/height/:height", get(block_by_height))
        .route("/tx/:txid", get(tx_by_id))
        .route("/address/:hash", get(address_info))
        .route("/transactions/new", post(new_transaction))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

//
// ─── STATUS (FIXED, HONEST) ───────────────────────
//

#[derive(Serialize)]
struct StatusResponse {
    height: u64,
    blocks: usize,
    utxos: usize,
    mempool: usize,

    // 🔒 CONSENSUS TRUTH
    total_issued: u64,

    // 🔒 LEDGER STATE
    utxo_supply: u64,

    // 🔒 ECONOMIC REALITY
    circulating_supply: u64,
}

async fn status(State(state): State<AppState>) -> Json<StatusResponse> {
    let c = state.chain.lock().unwrap();
    let height = c.height();

    // 1️⃣ TOTAL ISSUED (historical, independent of UTXOs)
    let mut total_issued = 0u64;
    for b in &c.blocks {
        total_issued = total_issued.saturating_add(
            block_reward(b.header.height)
        );
    }

    // 2️⃣ UTXO SUPPLY (current ledger state)
    let mut utxo_supply = 0u64;
    let mut circulating = 0u64;

    for u in c.utxos.values() {
        utxo_supply = utxo_supply.saturating_add(u.value);

        if !u.is_coinbase || height >= u.height + COINBASE_MATURITY {
            circulating = circulating.saturating_add(u.value);
        }
    }

    Json(StatusResponse {
        height,
        blocks: c.blocks.len(),
        utxos: c.utxos.len(),
        mempool: c.mempool.len(),

        total_issued,
        utxo_supply,
        circulating_supply: circulating,
    })
}

//
// ─── BLOCKS ───────────────────────────────────────
//

#[derive(Serialize)]
struct BlockResponse {
    height: u64,
    hash: String,
    txs: usize,
}

async fn blocks(State(state): State<AppState>) -> Json<Vec<BlockResponse>> {
    let c = state.chain.lock().unwrap();
    Json(
        c.blocks
            .iter()
            .map(|b| BlockResponse {
                height: b.header.height,
                hash: hex(&b.hash),
                txs: b.transactions.len(),
            })
            .collect(),
    )
}

async fn block_by_height(
    State(state): State<AppState>,
    Path(height): Path<u64>,
) -> impl IntoResponse {
    let c = state.chain.lock().unwrap();
    match c.blocks.iter().find(|b| b.header.height == height) {
        Some(b) => Json(BlockResponse {
            height,
            hash: hex(&b.hash),
            txs: b.transactions.len(),
        })
        .into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

//
// ─── TRANSACTIONS ─────────────────────────────────
//

#[derive(Serialize)]
struct TxResponse {
    txid: String,
    inputs: usize,
    outputs: usize,
}

async fn tx_by_id(
    State(state): State<AppState>,
    Path(txid): Path<String>,
) -> impl IntoResponse {
    let c = state.chain.lock().unwrap();
    for block in &c.blocks {
        for tx in &block.transactions {
            if hex(&tx.txid()) == txid {
                return Json(TxResponse {
                    txid,
                    inputs: tx.inputs.len(),
                    outputs: tx.outputs.len(),
                })
                .into_response();
            }
        }
    }
    StatusCode::NOT_FOUND.into_response()
}

//
// ─── NEW TRANSACTION (MEMPOOL) ────────────────────
//

#[derive(Deserialize)]
struct NewTxRequest {
    from: String,
    to: String,
    amount: u64,
}

async fn new_transaction(
    State(state): State<AppState>,
    Json(req): Json<NewTxRequest>,
) -> impl IntoResponse {
    let mut chain = state.chain.lock().unwrap();

    let from = match hex::decode(&req.from) {
        Ok(v) => v,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid sender").into_response(),
    };

    let to = match hex::decode(&req.to) {
        Ok(v) => v,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid receiver").into_response(),
    };

    match chain.create_transaction(from, to, req.amount) {
        Ok(tx) => {
            let txid = hex(&tx.txid());
            chain.mempool.push(tx);
            (
                StatusCode::OK,
                format!("Transaction added to mempool: {}", txid),
            )
                .into_response()
        }
        Err(e) => (
            StatusCode::BAD_REQUEST,
            format!("Transaction failed: {}", e),
        )
            .into_response(),
    }
}

//
// ─── ADDRESS INFO ─────────────────────────────────
//

#[derive(Serialize)]
struct AddressResponse {
    total: u64,
    spendable: u64,
    locked: u64,
    utxos: usize,
}

async fn address_info(
    State(state): State<AppState>,
    Path(hash): Path<String>,
) -> Json<AddressResponse> {
    let c = state.chain.lock().unwrap();
    let height = c.height();

    let mut total = 0u64;
    let mut spendable = 0u64;
    let mut locked = 0u64;
    let mut count = 0usize;

    for u in c.utxos.values() {
        if hex(&u.pubkey_hash) != hash {
            continue;
        }

        total += u.value;
        count += 1;

        if !u.is_coinbase {
            spendable += u.value;
        } else if height >= u.height + COINBASE_MATURITY {
            spendable += u.value;
        } else {
            locked += u.value;
        }
    }

    Json(AddressResponse {
        total,
        spendable,
        locked,
        utxos: count,
    })
}

//
// ─── HELPER ───────────────────────────────────────
//

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}
