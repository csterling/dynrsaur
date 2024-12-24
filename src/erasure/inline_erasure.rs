//! TODO: Use const generics to restrict `T` to validly-sized/aligned types once stable.

use std::mem::MaybeUninit;
use const_panic::concat_panic;
use crate::align::{AlignedBytes, Alignment, ValidAlignment};
use crate::erasure::Erasure;

#[repr(transparent)]
pub struct InlineErasure<const SIZE: usize, const ALIGN: usize>
    where Alignment<ALIGN>: ValidAlignment
{
    bytes: AlignedBytes<SIZE, ALIGN>
}

impl<
    const SIZE: usize,
    const ALIGN: usize
> InlineErasure<SIZE, ALIGN>
    where Alignment<ALIGN>: ValidAlignment
{
    pub const fn new<T>(value: T) -> Self {
        Self::check_size_and_align_of::<T>();
        
        let mut bytes = AlignedBytes::zeroed();

        unsafe { std::ptr::write(bytes.as_bytes_mut() as *mut u8 as *mut T, value) }
        
        Self { bytes }
    }
    
    const fn check_size_and_align_of<T>() {
        if size_of::<T>() > SIZE {
            // TODO: Add type_name::<T> once const-stable
            concat_panic!(
                "Size (",
                size_of::<T>(),
                " bytes) exceeds size of buffer (",
                SIZE,
                " bytes)"
            )
        }

        if align_of::<T>() > ALIGN {
            // TODO: Add type_name::<T> once const-stable
            concat_panic!(
                "Align (",
                align_of::<T>(),
                " bytes) exceeds align of buffer (",
                ALIGN,
                " bytes)"
            )
        }
    }
}

impl<
    const SIZE: usize,
    const ALIGN: usize,
    T
> Erasure<T> for InlineErasure<SIZE, ALIGN>
    where Alignment<ALIGN>: ValidAlignment
{
    unsafe fn downcast(self) -> T {
        Self::check_size_and_align_of::<T>();
        
        let mut uninitialized = MaybeUninit::<T>::uninit();
        
        std::ptr::copy_nonoverlapping(
            self.bytes.as_bytes() as *const u8 as *const T,
            uninitialized.as_mut_ptr(),
            1
        );
        
        uninitialized.assume_init()
    }
}

impl<
    'borrow,
    const SIZE: usize,
    const ALIGN: usize,
    T: 'borrow
> Erasure<&'borrow mut T> for &'borrow mut InlineErasure<SIZE, ALIGN>
    where Alignment<ALIGN>: ValidAlignment 
{
    unsafe fn downcast(self) -> &'borrow mut T {
        &mut *(self as *mut InlineErasure<SIZE, ALIGN> as *mut T)
    }
}

impl<
    'borrow,
    const SIZE: usize,
    const ALIGN: usize,
    T: 'borrow
> Erasure<&'borrow T> for &'borrow InlineErasure<SIZE, ALIGN>
where Alignment<ALIGN>: ValidAlignment
{
    unsafe fn downcast(self) -> &'borrow T {
        &*(self as *const InlineErasure<SIZE, ALIGN> as *const T)
    }
}