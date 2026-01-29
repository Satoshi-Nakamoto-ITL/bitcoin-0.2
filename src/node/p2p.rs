use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use crate::core::block::Block;
use crate::core::transaction::Transaction;
use crate::core::chain::Blockchain;
use crate::validation::validate_transaction;
use crate::node::message::{NetworkMessage, PROTOCOL_VERSION};
use crate::node::transport::Transport;

pub struct P2PNetwork {
    transport: Arc<dyn Transport>,
    chain: Arc<Mutex<Blockchain>>,
}

impl P2PNetwork {
    pub fn new(
        transport: Arc<dyn Transport>,
        chain: Arc<Mutex<Blockchain>>,
    ) -> Self {
        Self { transport, chain }
    }

    pub fn on_receive(&self, addr: SocketAddr, data: Vec<u8>) {
        let msg: NetworkMessage = match bincode::deserialize(&data) {
            Ok(m) => m,
            Err(_) => return,
        };

        match msg {
            NetworkMessage::Hello { version, height, .. } => {
                if version != PROTOCOL_VERSION {
                    return;
                }

                let local_height = self.chain.lock().unwrap().height();
                if height > local_height {
                    self.send(
                        addr,
                        &NetworkMessage::SyncRequest {
                            from_height: local_height,
                        },
                    );
                }
            }

            NetworkMessage::SyncRequest { from_height } => {
                let c = self.chain.lock().unwrap();
                for b in c.blocks.iter().skip(from_height as usize) {
                    self.send(addr, &NetworkMessage::Block(b.clone()));
                }
            }

            NetworkMessage::Block(block) => {
                self.chain
                    .lock()
                    .unwrap()
                    .validate_and_add_block(block);
            }

            NetworkMessage::Transaction(tx) => {
                let c = self.chain.lock().unwrap();
                let _ = validate_transaction(&tx, &c.utxos, c.height());
            }

            NetworkMessage::Ping => {
                self.send(addr, &NetworkMessage::Pong);
            }

            _ => {}
        }
    }

    fn send(&self, addr: SocketAddr, msg: &NetworkMessage) {
        let data = bincode::serialize(msg).unwrap();
        self.transport.send(&addr, &data);
    }

    pub fn broadcast_block(&self, block: &Block) {
        let data =
            bincode::serialize(&NetworkMessage::Block(block.clone())).unwrap();
        self.transport.broadcast(&data);
    }

    pub fn broadcast_transaction(&self, tx: &Transaction) {
        let data = bincode::serialize(
            &NetworkMessage::Transaction(tx.clone()),
        )
        .unwrap();
        self.transport.broadcast(&data);
    }

    pub fn peer_count(&self) -> usize {
        self.transport.peers().len()
    }
}
