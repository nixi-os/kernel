//! Devices

pub mod block;

use block::BlockDevice;

use alloc::collections::BTreeMap;
use alloc::sync::Arc;

use spin::{Lazy, Mutex};

static DEVICES: Lazy<Mutex<DeviceManager>> = Lazy::new(|| Mutex::new(DeviceManager::new()));

/// Call a closure with the device manager as an argument
pub fn with_device_manager<R, F: FnOnce(&mut DeviceManager) -> R>(f: F) -> R {
    f(&mut DEVICES.lock())
}

/// The device id is a unique identifier for a device
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DeviceId(u64);

impl From<u64> for DeviceId {
    fn from(value: u64) -> DeviceId {
        DeviceId(value)
    }
}

/// A device register
pub struct DeviceRegister<T> {
    devices: BTreeMap<DeviceId, T>,
    next_id: u64,
}

impl<T> DeviceRegister<T> {
    /// Create a new device manager
    pub fn new() -> DeviceRegister<T> {
        DeviceRegister {
            devices: BTreeMap::new(),
            next_id: 0,
        }
    }

    /// Register a new device
    pub fn register(&mut self, device: T) -> DeviceId {
        self.next_id += 1;

        let device_id = DeviceId::from(self.next_id);

        self.devices.insert(device_id, device);

        device_id
    }
}

/// The device manager stores device registers for all device types
pub struct DeviceManager {
    block_devices: DeviceRegister<Arc<dyn BlockDevice + Send + Sync>>,
}

impl DeviceManager {
    /// Create a new device manager
    pub fn new() -> DeviceManager {
        DeviceManager {
            block_devices: DeviceRegister::new(),
        }
    }
}
