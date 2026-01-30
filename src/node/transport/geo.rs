use std::net::{UdpSocket, SocketAddr};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

/// GEO / Mesh transport (LAN-based)
///
/// Uses UDP broadcast to:
/// - Discover peers on the local network
/// - Exchange raw Bitcoin P2P messages
///
/// This transport is:
/// - Internet-independent
/// - Consensus-agnostic
/// - Store-and-forward friendly
pub struct GeoTransport;

impl GeoTransport {
    /// Start GEO / LAN mesh transport
    ///
    /// Example bind:
    /// - "0.0.0.0:9333"
    pub fn start(
        bind: &str,
        on_receive: Arc<dyn Fn(SocketAddr, Vec<u8>) + Send + Sync>,
    ) {
        let socket = UdpSocket::bind(bind)
            .expect("GEO UDP bind failed");

        socket
            .set_broadcast(true)
            .expect("Failed to enable broadcast");

        socket
            .set_read_timeout(Some(Duration::from_secs(2)))
            .ok();

        println!("🌍 GEO mesh listening on {}", bind);

        // ───────── Receiver thread ─────────
        let recv_socket = socket.try_clone().unwrap();
        let on_receive_recv = Arc::clone(&on_receive);

        thread::spawn(move || {
            let mut buf = vec![0u8; 1024 * 1024];

            loop {
                match recv_socket.recv_from(&mut buf) {
                    Ok((n, src)) => {
                        // Feed bytes directly into P2P
                        (on_receive_recv)(src, buf[..n].to_vec());
                    }
                    Err(_) => {
                        // Timeout or temporary error
                        thread::sleep(Duration::from_millis(100));
                    }
                }
            }
        });

        // ───────── Announce thread ─────────
        thread::spawn(move || {
            let broadcast_addr: SocketAddr =
                "255.255.255.255:9333".parse().unwrap();

            loop {
                // Minimal presence announcement
                let announce = b"GEO_BITCOIN_NODE";

                let _ = socket.send_to(announce, broadcast_addr);

                thread::sleep(Duration::from_secs(5));
            }
        });
    }
}
