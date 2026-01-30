use std::net::SocketAddr;

// ───────── Transport implementations ─────────
pub mod tcp;
pub mod bluetooth;
pub mod satellite;
pub mod geo;
pub mod offline;

// ───────── Transport trait ─────────
pub trait Transport: Send + Sync {
    fn send(&self, addr: &SocketAddr, data: &[u8]);
    fn broadcast(&self, data: &[u8]);
    fn peers(&self) -> Vec<SocketAddr>;
}
