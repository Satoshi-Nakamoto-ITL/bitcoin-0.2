use std::sync::Arc;
use std::time::Duration;
use std::net::SocketAddr;

use btleplug::api::{
    Central, Manager as _, Peripheral as _, ScanFilter, CharPropFlags,
};
use btleplug::platform::Manager;
use futures::stream::StreamExt;
use uuid::Uuid;

/// BLE service & characteristic UUIDs
/// These MUST stay constant for network compatibility
const BITCOIN_BLE_SERVICE: Uuid =
    Uuid::from_u128(0xaaaaaaaa_bbbb_cccc_dddd_eeeeeeeeeeee);
const BITCOIN_BLE_CHAR: Uuid =
    Uuid::from_u128(0xffffffff_1111_2222_3333_444444444444);

/// Real Bluetooth Low Energy transport (receive-first)
pub struct BluetoothTransport;

impl BluetoothTransport {
    /// Start BLE listener
    ///
    /// Receives raw NetworkMessage bytes over BLE and injects
    /// them into the normal P2P pipeline via on_receive.
    pub async fn start(
        on_receive: Arc<dyn Fn(SocketAddr, Vec<u8>) + Send + Sync>,
    ) {
        let manager = Manager::new().await
            .expect("BLE manager failed");

        let adapters = manager.adapters().await
            .expect("No BLE adapters found");

        let central = adapters
            .into_iter()
            .next()
            .expect("No BLE adapter available");

        central
            .start_scan(ScanFilter::default())
            .await
            .expect("BLE scan failed");

        println!("🔵 BLE scanning started");

        loop {
            let peripherals = central
                .peripherals()
                .await
                .unwrap_or_default();

            for peripheral in peripherals {
                if let Ok(Some(props)) = peripheral.properties().await {
                    // ✅ FIX: services is Vec<Uuid>, not Option
                    if props.services.contains(&BITCOIN_BLE_SERVICE) {
                        if peripheral.connect().await.is_ok() {
                            let _ = peripheral.discover_services().await;

                            for characteristic in peripheral.characteristics() {
                                if characteristic.uuid == BITCOIN_BLE_CHAR
                                    && characteristic
                                        .properties
                                        .contains(CharPropFlags::NOTIFY)
                                {
                                    let _ =
                                        peripheral.subscribe(&characteristic).await;

                                    let mut notifications =
                                        peripheral.notifications().await.unwrap();

                                    let on_receive = Arc::clone(&on_receive);

                                    tokio::spawn(async move {
                                        while let Some(data) =
                                            notifications.next().await
                                        {
                                            // Dummy address for BLE source
                                            let addr: SocketAddr =
                                                "0.0.0.0:0".parse().unwrap();

                                            (on_receive)(addr, data.value);
                                        }
                                    });
                                }
                            }
                        }
                    }
                }
            }

            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }
}
