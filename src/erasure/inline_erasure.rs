use std::mem::MaybeUninit;
use const_panic::concat_panic;
use crate::align::{AlignedBytes, Alignment, ValidAlignment};
use crate::erasure::{Erase, Erasure};

/// An erasure of some underlying type that exists inline (i.e. the data for the erased-type value
/// is within this struct, as opposed to an indirection to the data).
/// 
/// The `SIZE`/`ALIGN` const generics are the maximum size/alignment of the underlying type.
/// Currently stable Rust can't compare these to [size_of]/[align_of] in const generics, so
/// enforcing this is a run-time check.
/// 
/// TODO: Use const generics to restrict `T` to validly-sized/aligned types once stable.
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
    /// Erases the given `value` inline.
    pub const fn new<T>(value: T) -> Self {
        // TODO: Use const generics to restrict `T` to validly-sized/aligned types once stable.
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
    unsafe fn downcast_unchecked(self) -> T {
        // TODO: Use const generics to restrict `T` to validly-sized/aligned types once stable.
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
    const SIZE: usize,
    const ALIGN: usize,
    T
> Erase<T> for InlineErasure<SIZE, ALIGN>
    where Alignment<ALIGN>: ValidAlignment
{
    fn erase(value: T) -> Self {
        Self::new(value)
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
    unsafe fn downcast_unchecked(self) -> &'borrow mut T {
        // TODO: Use const generics to restrict `T` to validly-sized/aligned types once stable.
        InlineErasure::<SIZE, ALIGN>::check_size_and_align_of::<T>();
        
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
    unsafe fn downcast_unchecked(self) -> &'borrow T {
        // TODO: Use const generics to restrict `T` to validly-sized/aligned types once stable.
        InlineErasure::<SIZE, ALIGN>::check_size_and_align_of::<T>();
        
        &*(self as *const InlineErasure<SIZE, ALIGN> as *const T)
    }
}