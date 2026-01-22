use tokio::net::TcpListener;
use serde::Serialize;
use std::sync::{Arc, Mutex};
use std::net::SocketAddr;

use axum::{
    Router,
    Json,
    routing::{get, post},
    extract::State,
};

use crate::chain::Blockchain;
use crate::mempool::Mempool;
use crate::transaction::Transaction;

#[derive(Clone)]
struct AppState {
    chain: Arc<Mutex<Blockchain>>,
    mempool: Arc<Mutex<Mempool>>,
}

#[derive(Serialize)]
struct StatusResponse {
    height: usize,
    difficulty: u32,
    utxos: usize,
    mempool: usize,
}

#[derive(Serialize)]
struct MempoolStatusResponse {
    size: usize,
}

#[derive(Serialize)]
struct SubmitTxResponse {
    accepted: bool,
}

pub async fn start_api(
    chain: Arc<Mutex<Blockchain>>,
    mempool: Arc<Mutex<Mempool>>,
    port: u16,
) {
    let state = AppState { chain, mempool };

    let app = Router::new()
        .route("/status", get(status))
        .route("/tx", post(submit_tx))
        .route("/mempool", get(mempool_status))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("🌐 API running at http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn status(
    State(state): State<AppState>,
) -> Json<StatusResponse> {
    let c = state.chain.lock().unwrap();
    let m = state.mempool.lock().unwrap();

    Json(StatusResponse {
        height: c.blocks.len(),
        difficulty: c.blocks.last().map(|b| b.header.difficulty).unwrap_or(0),
        utxos: c.utxos.len(),
        mempool: m.size(),
    })
}

async fn submit_tx(
    State(state): State<AppState>,
    Json(tx): Json<Transaction>,
) -> Json<SubmitTxResponse> {
    let c = state.chain.lock().unwrap();
    let mut mp = state.mempool.lock().unwrap();

    let accepted = mp.add_transaction(tx, c.utxos());
    Json(SubmitTxResponse { accepted })
}

async fn mempool_status(
    State(state): State<AppState>,
) -> Json<MempoolStatusResponse> {
    Json(MempoolStatusResponse {
        size: state.mempool.lock().unwrap().size(),
    })
}
