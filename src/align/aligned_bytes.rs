use std::ops::{Deref, DerefMut};
use crate::align::{ValidAlignment, Alignment};

pub struct AlignedBytes<
    const SIZE: usize,
    const ALIGN: usize
> 
    where Alignment<ALIGN>: ValidAlignment
{
    align: Alignment<ALIGN>,
    bytes: [u8; SIZE]
}


impl<
    const SIZE: usize,
    const ALIGN: usize
> AlignedBytes<SIZE, ALIGN>
    where Alignment<ALIGN>: ValidAlignment
{
    pub const fn zeroed() -> Self {
        AlignedBytes {
            align: Alignment::new(),
            bytes: [0; SIZE]
        }
    }
    pub const fn as_bytes(&self) -> &[u8; SIZE] {
        &self.bytes
    }
    
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
        AlignedBytes {
            align: Alignment::default(),
            bytes: [0; SIZE]
        }
    }
}

impl<
    const SIZE: usize,
    const ALIGN: usize,
    T: Into<[u8; SIZE]>
> From<T> for AlignedBytes<SIZE, ALIGN>
where Alignment<ALIGN>: ValidAlignment
{
    fn from(value: T) -> Self {
        Self {
            align: Default::default(),
            bytes: value.into()
        }
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