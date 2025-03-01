use std::any::Any;
use std::panic::{RefUnwindSafe, UnwindSafe};
use crate::erasure::Erasure;
use crate::for_all_combinations;

/// [Erasure]-type which can be constructed by taking ownership of a value.
/// 
/// Generic Parameters:
/// - `T`: The underlying erased type.
pub trait Erase<T>: Erasure<T> {
    /// Creates an instance of this type of [erasure](Erasure) from a value of type `T`.
    fn erase(value: T) -> Self;
}

macro_rules! impl_erase_for_any {
    ($($traits:ident),*) => {
        impl<T: 'static$( + $traits)*> Erase<T> for Box<dyn Any$( + $traits)*> {
            fn erase(value: T) -> Self {
                Box::new(value) as Box<dyn Any$( + $traits)*>
            }
        }
        
        impl<'lifetime, T: 'static$( + $traits)*> Erase<&'lifetime T> for &'lifetime (dyn Any$( + $traits)*) {
            fn erase(value: &'lifetime T) -> Self {
                value as &(dyn Any$( + $traits)*)
            }
        }
        
        impl<'lifetime, T: 'static$( + $traits)*> Erase<&'lifetime mut T> for &'lifetime mut (dyn Any$( + $traits)*){
            fn erase(value: &'lifetime mut T) -> Self {
                value as &mut (dyn Any$( + $traits)*)
            }
        }
    };
}

for_all_combinations!(impl_erase_for_any => Send, Sync, Unpin, UnwindSafe, RefUnwindSafe);
