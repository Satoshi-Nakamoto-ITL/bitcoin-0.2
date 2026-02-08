use tokio::net::TcpListener;
use serde::Serialize;
use std::sync::{Arc, Mutex};
use std::net::SocketAddr;

use axum::{
    Router,
    Json,
    routing::get,
    extract::State,
    http::StatusCode,
};

use crate::core::chain::Blockchain;

const COINBASE_MATURITY: u64 = 100;

#[derive(Clone)]
struct AppState {
    chain: Arc<Mutex<Blockchain>>,
}

/* ───────── API START ───────── */

pub async fn start_api(chain: Arc<Mutex<Blockchain>>, port: u16) {
    println!("🌐 API starting on http://127.0.0.1:{}/status", port);

    let state = AppState { chain };

    let app = Router::new()
        .route("/status", get(status))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr)
        .await
        .expect("API bind failed");

    axum::serve(listener, app)
        .await
        .expect("API server crashed");
}

/* ───────── STATUS ───────── */

#[derive(Serialize)]
struct StatusResponse {
    height: u64,
    blocks: usize,
    utxos: usize,
    mempool: usize,

    // supply-level view (node-owned UTXOs only)
    total_supply: u64,
    spendable_supply: u64,
    locked_supply: u64,
    next_spendable_height: Option<u64>,
}

async fn status(
    State(state): State<AppState>,
) -> Result<Json<StatusResponse>, StatusCode> {
    let chain = state.chain.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let height = chain.height();

    let mut total = 0u64;
    let mut spendable = 0u64;
    let mut locked = 0u64;
    let mut next_spendable_height: Option<u64> = None;

    for utxo in chain.utxos.values() {
        total += utxo.value;

        if !utxo.is_coinbase {
            spendable += utxo.value;
        } else {
            let mature_height = utxo.height + COINBASE_MATURITY;

            if height >= mature_height {
                spendable += utxo.value;
            } else {
                locked += utxo.value;
                next_spendable_height = match next_spendable_height {
                    Some(h) => Some(h.min(mature_height)),
                    None => Some(mature_height),
                };
            }
        }
    }

    Ok(Json(StatusResponse {
        height,
        blocks: chain.blocks.len(),
        utxos: chain.utxos.len(),
        mempool: chain.mempool.len(),
        total_supply: total,
        spendable_supply: spendable,
        locked_supply: locked,
        next_spendable_height,
    }))
}
