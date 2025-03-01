use std::any::Any;
use std::panic::{RefUnwindSafe, UnwindSafe};
use crate::erasure::defer_downcast_unchecked_to_try_downcast;
use crate::for_all_combinations;

/// Trait implemented by types that represent type-erased containers for other types.
/// 
/// Generic Parameters:
/// - `T`: a possible underlying type that could have been erased.
pub trait Erasure<T>: Sized {
    /// Downcast this erasure into its underlying type `T`.
    /// 
    /// SAFETY: Caller must ensure that the erasure really is a `T` underneath.
    unsafe fn downcast_unchecked(self) -> T;
}

macro_rules! impl_erasure_for_any {
    ($($traits:ident),*) => {
        impl<T: 'static$( + $traits)*> Erasure<T> for Box<dyn Any$( + $traits)*> {
            unsafe fn downcast_unchecked(self) -> T {
                defer_downcast_unchecked_to_try_downcast(self)
            }
        }
        
        impl<'borrow, T: 'static$( + $traits)*> Erasure<&'borrow T> for &'borrow (dyn Any$( + $traits)*) {
            unsafe fn downcast_unchecked(self) -> &'borrow T {
                defer_downcast_unchecked_to_try_downcast(self)
            }
        }
        
        impl<'borrow, T: 'static$( + $traits)*> Erasure<&'borrow mut T> for &'borrow mut (dyn Any$( + $traits)*) {
            unsafe fn downcast_unchecked(self) -> &'borrow mut T {
                defer_downcast_unchecked_to_try_downcast(self)
            }
        }
    };
}

for_all_combinations!(impl_erasure_for_any => Send, Sync, Unpin, UnwindSafe, RefUnwindSafe);