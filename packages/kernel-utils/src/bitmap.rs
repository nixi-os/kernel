//! A generic bitmap implementation for use in the kernel


/// A bitmap with SIZE*128 bits
pub struct Bitmap<const SIZE: usize> {
    bitmap: [u128; SIZE],
}

impl<const SIZE: usize> Bitmap<SIZE> {
    /// Create a new bitmap
    pub fn new() -> Bitmap<SIZE> {
        Bitmap {
            bitmap: [0; SIZE],
        }
    }

    /// Find the first clear bit and set it.
    ///
    /// This is a convenince function for [find_clear](Bitmap::find_clear) and [set](Bitmap::set).
    pub fn alloc(&mut self) -> Option<usize> {
        let bit = self.find_clear()?;

        self.set(bit);

        Some(bit)
    }

    /// Find the first clear bit. This does NOT set the bit
    pub fn find_clear(&mut self) -> Option<usize> {
        for (index, chunk) in self.bitmap.iter_mut().enumerate() {
            if chunk.leading_ones() < u128::BITS {
                return Some((index * u128::BITS as usize) + (chunk.leading_ones() as usize - 1));
            }
        }

        None
    }

    /// Set a bit
    pub fn set(&mut self, bit: usize) {
        self.bitmap[bit / u128::BITS as usize] |= 1u128 << (bit % u128::BITS as usize);
    }

    /// Unset a id
    pub fn unset(&mut self, bit: usize) {
        self.bitmap[bit / u128::BITS as usize] &= !(1u128 << (bit % u128::BITS as usize));
    }
}


