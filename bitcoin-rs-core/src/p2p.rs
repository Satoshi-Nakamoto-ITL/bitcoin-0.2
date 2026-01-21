use std::collections::HashMap;
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

use serde::{Serialize, Deserialize};

use crate::block::Block;
use crate::transaction::Transaction;
use crate::chain::Blockchain;

#[derive(Clone, Serialize, Deserialize)]
pub enum P2PMessage {
    SyncRequest { from_height: u64 },
    Block(Block),
    Transaction(Transaction),
    Ping,
    Pong,
}

pub struct PeerNode {
    pub address: SocketAddr,
    pub last_seen: i64,
    pub stream: TcpStream,
}

pub struct P2PNetwork {
    peers: Arc<Mutex<HashMap<String, PeerNode>>>,
    listener_addr: SocketAddr,
    chain: Arc<Mutex<Blockchain>>,
}

impl P2PNetwork {
    pub fn new(chain: Arc<Mutex<Blockchain>>) -> Self {
        let listener = TcpListener::bind("0.0.0.0:0")
            .expect("Failed to bind P2P listener");

        listener.set_nonblocking(true).unwrap();
        let listener_addr = listener.local_addr().unwrap();

        println!("🔗 P2P listening on {}", listener_addr);

        let peers = Arc::new(Mutex::new(HashMap::new()));
        let peers_clone = Arc::clone(&peers);
        let chain_clone = Arc::clone(&chain);

        thread::spawn(move || loop {
            match listener.accept() {
                Ok((stream, addr)) => {
                    stream.set_read_timeout(Some(Duration::from_secs(30))).ok();

                    peers_clone.lock().unwrap().insert(
                        addr.to_string(),
                        PeerNode {
                            address: addr,
                            last_seen: now(),
                            stream: stream.try_clone().unwrap(),
                        },
                    );

                    let peers_inner = Arc::clone(&peers_clone);
                    let chain_inner = Arc::clone(&chain_clone);

                    thread::spawn(move || {
                        Self::handle_peer(stream, peers_inner, chain_inner);
                    });
                }
                Err(_) => {
                    thread::sleep(Duration::from_millis(100));
                }
            }
        });

        Self {
            peers,
            listener_addr,
            chain,
        }
    }

    pub fn local_addr(&self) -> SocketAddr {
        self.listener_addr
    }

    fn handle_peer(
        mut stream: TcpStream,
        peers: Arc<Mutex<HashMap<String, PeerNode>>>,
        chain: Arc<Mutex<Blockchain>>,
    ) {
        let peer_addr = match stream.peer_addr() {
            Ok(a) => a.to_string(),
            Err(_) => return,
        };

        loop {
            let mut buffer = vec![0u8; 4 * 1024 * 1024];

            match stream.read(&mut buffer) {
                Ok(n) if n > 0 => {
                    // update last_seen on any message
                    if let Some(peer) = peers.lock().unwrap().get_mut(&peer_addr) {
                        peer.last_seen = now();
                        let _ = peer.address; // read address (keeps field meaningful)
                    }

                    let msg: P2PMessage = match bincode::deserialize(&buffer[..n]) {
                        Ok(m) => m,
                        Err(_) => continue,
                    };

                    match msg {
                        P2PMessage::SyncRequest { from_height } => {
                            let chain = chain.lock().unwrap();
                            for block in chain.blocks.iter().skip(from_height as usize) {
                                let data = bincode::serialize(
                                    &P2PMessage::Block(block.clone())
                                ).unwrap();
                                let _ = stream.write_all(&data);
                            }
                        }

                        P2PMessage::Block(block) => {
                            let mut chain = chain.lock().unwrap();
                            chain.validate_and_add_block(block);
                        }

                        P2PMessage::Transaction(_) => {}

                        P2PMessage::Ping => {
                            let data = bincode::serialize(&P2PMessage::Pong).unwrap();
                            let _ = stream.write_all(&data);
                        }

                        P2PMessage::Pong => {}
                    }
                }
                _ => break,
            }
        }
    }

    pub fn request_sync(&self) {
        let height = self.chain.lock().unwrap().height();
        let msg = P2PMessage::SyncRequest { from_height: height };
        let data = bincode::serialize(&msg).unwrap();

        let mut peers = self.peers.lock().unwrap();
        for peer in peers.values_mut() {
            let _ = peer.stream.write_all(&data);
        }
    }

    pub fn broadcast_block(&self, block: &Block) {
        let msg = P2PMessage::Block(block.clone());
        let data = bincode::serialize(&msg).unwrap();

        let mut peers = self.peers.lock().unwrap();
        for peer in peers.values_mut() {
            let _ = peer.stream.write_all(&data);
        }
    }

    pub fn peer_count(&self) -> usize {
        self.peers.lock().unwrap().len()
    }
}

fn now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}
