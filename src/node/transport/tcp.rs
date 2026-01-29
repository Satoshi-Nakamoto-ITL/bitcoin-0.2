use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::node::transport::Transport;

const MAX_MESSAGE_SIZE: usize = 1 * 1024 * 1024;

pub struct TcpTransport {
    peers: Arc<Mutex<HashMap<SocketAddr, TcpStream>>>,
}

impl TcpTransport {
    pub fn new(
        bind: &str,
        on_receive: Arc<dyn Fn(SocketAddr, Vec<u8>) + Send + Sync>,
    ) -> Arc<Self> {
        let listener = TcpListener::bind(bind).expect("TCP bind failed");
        listener.set_nonblocking(true).unwrap();

        let peers = Arc::new(Mutex::new(HashMap::new()));
        let peers_accept = Arc::clone(&peers);
        let on_receive = Arc::clone(&on_receive);

        thread::spawn(move || loop {
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    stream
                        .set_read_timeout(Some(Duration::from_secs(30)))
                        .ok();

                    peers_accept
                        .lock()
                        .unwrap()
                        .insert(addr, stream.try_clone().unwrap());

                    let peers_inner = Arc::clone(&peers_accept);
                    let on_receive = Arc::clone(&on_receive);

                    thread::spawn(move || {
                        let mut buf = vec![0u8; MAX_MESSAGE_SIZE];
                        loop {
                            match stream.read(&mut buf) {
                                Ok(0) | Err(_) => break,
                                Ok(n) => (on_receive)(addr, buf[..n].to_vec()),
                            }
                        }
                        peers_inner.lock().unwrap().remove(&addr);
                    });
                }
                Err(_) => thread::sleep(Duration::from_millis(50)),
            }
        });

        Arc::new(Self { peers })
    }

    pub fn connect(&self, addr: SocketAddr) {
        if let Ok(stream) = TcpStream::connect(addr) {
            self.peers.lock().unwrap().insert(addr, stream);
        }
    }
}

impl Transport for TcpTransport {
    fn send(&self, addr: &SocketAddr, data: &[u8]) {
        if let Some(s) = self.peers.lock().unwrap().get_mut(addr) {
            let _ = s.write_all(data);
        }
    }

    fn broadcast(&self, data: &[u8]) {
        for s in self.peers.lock().unwrap().values_mut() {
            let _ = s.write_all(data);
        }
    }

    fn peers(&self) -> Vec<SocketAddr> {
        self.peers.lock().unwrap().keys().cloned().collect()
    }
}
