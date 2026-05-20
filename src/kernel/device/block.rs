//! Block devices


/// A block device is a device which is written and read in blocks
pub trait BlockDevice {
    /// Read a block
    fn read(&self, lba: u64, buf: &mut [u8; 512]) -> Result<(), BlockError>;

    /// Write a block
    fn write(&self, lba: u64, buf: &[u8; 512]) -> Result<(), BlockError>;
}

/// A error for a block device
pub enum BlockError {
    InvalidLba,
}


