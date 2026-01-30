use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

/// Offline / Store-and-Forward transport
///
/// Allows Bitcoin messages to be exported to disk and later
/// re-imported on another node.
///
/// Designed for:
/// - USB / SD card transfer
/// - Air-gapped systems
/// - Delayed connectivity
pub struct OfflineTransport;

impl OfflineTransport {
    /// Export raw P2P message bytes to a file
    ///
    /// Example:
    /// OfflineTransport::export("offline.msg", &bytes)
    pub fn export(path: &str, data: &[u8]) {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .expect("Offline export open failed");

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Simple framing:
        // [timestamp u64][len u32][data]
        file.write_all(&timestamp.to_le_bytes()).unwrap();
        file.write_all(&(data.len() as u32).to_le_bytes()).unwrap();
        file.write_all(data).unwrap();
    }

    /// Import offline messages from a file
    ///
    /// Example:
    /// OfflineTransport::import("offline.msg", on_receive)
    pub fn import(
        path: &str,
        on_receive: Arc<dyn Fn(SocketAddr, Vec<u8>) + Send + Sync>,
    ) {
        let mut file = File::open(path)
            .expect("Offline import open failed");

        let mut buf = Vec::new();
        file.read_to_end(&mut buf).unwrap();

        let mut i = 0;
        while i + 12 <= buf.len() {
            let _timestamp =
                u64::from_le_bytes(buf[i..i + 8].try_into().unwrap());
            i += 8;

            let len =
                u32::from_le_bytes(buf[i..i + 4].try_into().unwrap()) as usize;
            i += 4;

            if i + len > buf.len() {
                break;
            }

            let data = buf[i..i + len].to_vec();
            i += len;

            // Dummy address for offline source
            let addr: SocketAddr = "0.0.0.0:0".parse().unwrap();
            (on_receive)(addr, data);
        }
    }
}
