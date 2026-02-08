use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

// Required imports from the project structure
use crate::core::block::Block;
use crate::core::chain::Blockchain;
use crate::validation::validate_transaction;
use crate::node::message::{NetworkMessage, PROTOCOL_VERSION};
use crate::node::transport::Transport;

/// The P2P Network Layer
/// Handles peer communication and message broadcasting
pub struct P2PNetwork {
    transport: Arc<dyn Transport>,
    chain: Arc<Mutex<Blockchain>>,
}

impl P2PNetwork {
    /// Initialize the P2P Network
    /// Matches the signature required by main.rs
    pub fn new(
        transport: Arc<dyn Transport>,
        chain: Arc<Mutex<Blockchain>>,
    ) -> Self {
        // System logs to show network status
        println!("> [SYSTEM] Initializing P2P Network Layer...");
        println!("> [INFO] Protocol Version: {}", PROTOCOL_VERSION);
        println!("> [STATUS] Node is active and listening...");

        Self { transport, chain }
    }

    /// Handle incoming messages from peers
    pub fn on_receive(&self, addr: SocketAddr, data: Vec<u8>) {
        // Deserialize message safely
        let msg: NetworkMessage = match bincode::deserialize(&data) {
            Ok(m) => m,
            Err(_) => {
                println!("> [WARN] Invalid packet received from {}", addr);
                return;
            }
        };

        // Process message with system logging
        match msg {
            NetworkMessage::Hello { version, height, .. } => {
                println!("> [NET] Handshake request from {} (Height: {})", addr, height);

                if version != PROTOCOL_VERSION {
                    println!("> [DENY] Protocol mismatch with {}", addr);
                    return;
                }

                let local_height = self.chain.lock().unwrap().height();
                if height > local_height {
                    println!("> [SYNC] Peer is ahead. Requesting blocks...");
                    self.send(addr, &NetworkMessage::SyncRequest { from_height: local_height });
                }
            }

            NetworkMessage::SyncRequest { from_height } => {
                println!("> [QUERY] Serving blocks from height {}", from_height);
                let c = self.chain.lock().unwrap();
                for b in c.blocks.iter().skip(from_height as usize) {
                    self.send(addr, &NetworkMessage::Block(b.clone()));
                }
            }

            NetworkMessage::Block(block) => {
                println!("> [BLOCK] New block received. Validating...");
                self.chain.lock().unwrap().validate_and_add_block(block);
                println!("> [SUCCESS] Block added to chain.");
            }

            NetworkMessage::Transaction(tx) => {
                println!("> [TX] Processing incoming transaction...");
                let c = self.chain.lock().unwrap();
                let _ = validate_transaction(&tx, &c.utxos, c.height());
            }

            NetworkMessage::Ping => {
                self.send(addr, &NetworkMessage::Pong);
            }

            _ => {}
        }
    }

    /// Helper function to send messages to a single peer
    fn send(&self, addr: SocketAddr, msg: &NetworkMessage) {
        if let Ok(data) = bincode::serialize(msg) {
            self.transport.send(&addr, &data);
        }
    }

    /// ✅ FIX: Broadcast a newly mined block to all peers
    pub fn broadcast_block(&self, block: &Block) {
        println!(
            "> [NET] Broadcasting block at height {}",
            block.header.height
        );

        let msg = NetworkMessage::Block(block.clone());

        if let Ok(data) = bincode::serialize(&msg) {
            self.transport.broadcast(&data);
        }
    }
}