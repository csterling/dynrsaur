use std::any::{type_name, Any};

/// Trait implemented by types that represent type-erased containers for other types. The
/// type-parameter `T` is a possible underlying type that could have been erased, hence the `unsafe`
/// on the [downcast](Erasure::downcast) method, as the caller must ensure that the correct type is
/// chosen.
pub trait Erasure<T>: Sized {
    unsafe fn downcast(self) -> T;
}

impl<T: 'static> Erasure<T> for Box<dyn Any> {
    unsafe fn downcast(self) -> T {
        match self.downcast::<T>() {
            Ok(v) => *v,
            Err(_) => panic!("Invalid downcast to '{}'", type_name::<T>()),
        }
    }
}

impl<'borrow, T: 'static> Erasure<&'borrow T> for &'borrow dyn Any {
    unsafe fn downcast(self) -> &'borrow T {
        match self.downcast_ref::<T>() {
            Some(v) => v,
            None => panic!("Invalid downcast to '{}'", type_name::<T>()),
        }
    }
}

impl<'borrow, T: 'static> Erasure<&'borrow mut T> for &'borrow mut dyn Any {
    unsafe fn downcast(self) -> &'borrow mut T {
        match self.downcast_mut::<T>() {
            Some(v) => v,
            None => panic!("Invalid downcast to '{}'", type_name::<T>()),
        }
    }
}
