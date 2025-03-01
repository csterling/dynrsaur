use std::any::Any;
use std::mem::transmute;
use std::ops::Deref;
use std::panic::{RefUnwindSafe, UnwindSafe};
use crate::erasure::Erasure;
use crate::for_all_combinations;

/// Indicates that the type is a potential erasure of `T`, and contains information
/// to determine if that is the case.
/// 
/// Rust disallows a blanket implementation of [Erasure], so it is required as a super-trait
/// instead, providing the default implementation 
/// [`defer_downcast_unchecked_to_try_downcast`](crate::erasure::defer_downcast_unchecked_to_try_downcast::defer_downcast_unchecked_to_try_downcast).  
///
/// Generic Parameters:
/// - `T`: a possible underlying type that could have been erased.
pub trait TaggedErasure<T>: Erasure<T> {
    /// Checks if this erasure has underlying type `T`.
    fn is(&self) -> bool;
    
    /// Downcast this erasure into a `T`, if the underlying type is such, returning `self`
    /// unchanged if not.
    fn try_downcast(self) -> Result<T, Self>;
}
macro_rules! impl_tagged_erasure_for_any {
    ($($traits:ident),*) => {
        impl<T: 'static$( + $traits)*> TaggedErasure<T> for Box<dyn Any$( + $traits)*> {
            fn is(&self) -> bool {
                <dyn Any>::is::<T>(self.deref())
            }
        
            fn try_downcast(self) -> Result<T, Self> {
                <Box<dyn Any>>::downcast(self)
                    .map(|boxed| *boxed)
                    // SAFETY: Err value is self
                    .map_err(|self_| unsafe { transmute(self_) })
            }
        }
        
        impl<'borrow, T: 'static$( + $traits)*> TaggedErasure<&'borrow T> for &'borrow (dyn Any$( + $traits)*) {
            fn is(&self) -> bool {
                <dyn Any>::is::<T>(*self)
            }
        
            fn try_downcast(self) -> Result<&'borrow T, Self> {
                <dyn Any>::downcast_ref(self)
                    .ok_or(self)
            }
        }
        
        impl<'borrow, T: 'static$( + $traits)*> TaggedErasure<&'borrow mut T> for &'borrow mut (dyn Any$( + $traits)*) {
            fn is(&self) -> bool {
                <dyn Any>::is::<T>(&**self)
            }
        
            fn try_downcast(self) -> Result<&'borrow mut T, Self> {
                if <dyn Any>::is::<T>(&*self) {
                    // TODO: Change to <dyn Any>::downcast_mut_unchecked when stable.
                    // SAFETY: Above if-statement
                    Ok(unsafe { &mut *(self as *mut dyn Any as *mut T) })
                } else {
                    Err(self)
                }
            }
        }
    };
}

for_all_combinations!(impl_tagged_erasure_for_any => Send, Sync, Unpin, UnwindSafe, RefUnwindSafe);
