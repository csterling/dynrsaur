use std::ops::{Deref, DerefMut};
use crate::align::Alignment;
use crate::align::valid_alignment::ValidAlignment;

/// A byte-array (`[u8; SIZE]`) that is aligned to `ALIGN` bytes.
pub struct AlignedBytes<
    const SIZE: usize,
    const ALIGN: usize
> 
    where Alignment<ALIGN>: ValidAlignment
{
    _align: Alignment<ALIGN>,
    bytes: [u8; SIZE]
}


impl<
    const SIZE: usize,
    const ALIGN: usize
> AlignedBytes<SIZE, ALIGN>
    where Alignment<ALIGN>: ValidAlignment
{
    /// Aligns the given byte-array.
    pub const fn from(bytes: [u8; SIZE]) -> Self {
        AlignedBytes {
            _align: Alignment::new(),
            bytes
        }
    }
    
    /// Creates an aligned byte-array, where all elements are zero.
    pub const fn zeroed() -> Self {
        Self::from([0; SIZE])
    }
    
    /// Borrows the underlying byte-array.
    pub const fn as_bytes(&self) -> &[u8; SIZE] {
        &self.bytes
    }
    
    /// Mutably borrows the underlying byte-array.
    pub const fn as_bytes_mut(&mut self) -> &mut [u8; SIZE] {
        &mut self.bytes
    }
}

impl<
    const SIZE: usize,
    const ALIGN: usize
> Default for AlignedBytes<SIZE, ALIGN>
    where Alignment<ALIGN>: ValidAlignment 
{
    fn default() -> Self {
        Self::zeroed()
    }
}

impl<
    const SIZE: usize,
    const ALIGN: usize,
> From<[u8; SIZE]> for AlignedBytes<SIZE, ALIGN>
    where Alignment<ALIGN>: ValidAlignment
{
    fn from(value: [u8; SIZE]) -> Self {
        Self::from(value)
    }
}

impl<
    const SIZE: usize,
    const ALIGN: usize
> From<AlignedBytes<SIZE, ALIGN>> for [u8; SIZE]
    where Alignment<ALIGN>: ValidAlignment
{
    fn from(value: AlignedBytes<SIZE, ALIGN>) -> Self {
        value.bytes
    }
}

impl<
    const SIZE: usize,
    const ALIGN: usize
> Deref for AlignedBytes<SIZE, ALIGN>
    where Alignment<ALIGN>: ValidAlignment
{
    type Target = [u8; SIZE];

    fn deref(&self) -> &Self::Target {
        &self.bytes
    }
}

impl<
    const SIZE: usize,
    const ALIGN: usize
> DerefMut for AlignedBytes<SIZE, ALIGN>
    where Alignment<ALIGN>: ValidAlignment
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.bytes
    }
}