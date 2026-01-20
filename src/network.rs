use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use crate::block::Block;
use crate::transaction::Transaction;
use crate::chain::Blockchain;
use crate::p2p::P2PNetwork as PrivateP2PNetwork;

pub struct P2PNetwork {
    inner: PrivateP2PNetwork,
}

impl P2PNetwork {
    pub fn new(listen_addr: SocketAddr, chain: Arc<Mutex<Blockchain>>) -> Self {
        let inner = PrivateP2PNetwork::new(listen_addr, chain);
        Self { inner }
    }

    pub fn request_sync(&self) {
        self.inner.request_sync();
    }

    pub fn broadcast_block(&self, block: &Block) {
        self.inner.broadcast_block(block);
    }

    pub fn broadcast_transaction(&self, _tx: &Transaction) {
        // v0.2: transaction gossip not enabled yet
    }

    pub fn peer_count(&self) -> usize {
        self.inner.peer_count()
    }
}
