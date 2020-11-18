//! HCI Hardware Abstraction Layer
//! Supports sending HCI commands to the HAL and receving
//! HCI events from the HAL
pub mod facade;
pub mod rootcanal_hal;

use thiserror::Error;
use tokio::sync::mpsc;

use bt_packet::{HciCommand, HciEvent, RawPacket};

/// H4 packet header size
const H4_HEADER_SIZE: usize = 1;

/// HAL interface
/// This is used by the HCI module to send commands to the
/// HAL and receive events from the HAL
pub struct HalExports {
    /// Transmit end of a channel used to send HCI commands
    pub cmd_tx: mpsc::UnboundedSender<HciCommand>,
    /// Receive end of a channel used to receive HCI events
    pub evt_rx: mpsc::UnboundedReceiver<HciEvent>,
    /// Transmit end of a channel used to send ACL data
    pub acl_tx: mpsc::UnboundedSender<RawPacket>,
    /// Receive end of a channel used to receive ACL data
    pub acl_rx: mpsc::UnboundedReceiver<RawPacket>,
}

/// HCI HAL
/// Receive HCI commands, send HCI events
pub struct Hal {
    /// Receive end of a channel used to receive HCI commands
    pub cmd_rx: mpsc::UnboundedReceiver<HciCommand>,
    /// Transmit end of a channel used to send HCI events
    pub evt_tx: mpsc::UnboundedSender<HciEvent>,
    /// Receive end of a channel used to send ACL data
    pub acl_rx: mpsc::UnboundedReceiver<RawPacket>,
    /// Transmit end of a channel used to receive ACL data
    pub acl_tx: mpsc::UnboundedSender<RawPacket>,
}

impl Hal {
    /// Create a new Hal instance
    pub fn new() -> (HalExports, Self) {
        let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();
        let (evt_tx, evt_rx) = mpsc::unbounded_channel();
        let (acl_down_tx, acl_down_rx) = mpsc::unbounded_channel();
        let (acl_up_tx, acl_up_rx) = mpsc::unbounded_channel();
        (
            HalExports {
                cmd_tx,
                evt_rx,
                acl_tx: acl_down_tx,
                acl_rx: acl_up_rx,
            },
            Self {
                cmd_rx,
                evt_tx,
                acl_rx: acl_down_rx,
                acl_tx: acl_up_tx,
            },
        )
    }
}

/// Result type
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// Errors that can be encountered while dealing with the HAL
#[derive(Error, Debug)]
pub enum HalError {
    /// Invalid rootcanal host error
    #[error("Invalid rootcanal host")]
    InvalidAddressError,
    /// Error while connecting to rootcanal
    #[error("Connection to rootcanal failed: {0}")]
    RootcanalConnectError(#[from] tokio::io::Error),
}
