use std::any::Any;
use crate::erasure::Erasure;

pub trait TaggedErasure<T>: Erasure<T> {
    fn try_downcast(self) -> Result<T, Self>;
}

impl<T: 'static> TaggedErasure<T> for Box<dyn Any> {
    fn try_downcast(self) -> Result<T, Self> {
        Self::downcast::<T>(self).map(|t| *t)
    }
}

impl<'borrow, T: 'static> TaggedErasure<&'borrow T> for &'borrow dyn Any {
    fn try_downcast(self) -> Result<&'borrow T, Self> {
        <dyn Any>::downcast_ref::<T>(self).ok_or(self)
    }
}

impl<'borrow, T: 'static> TaggedErasure<&'borrow mut T> for &'borrow mut dyn Any {
    fn try_downcast(self) -> Result<&'borrow mut T, Self> {
        let pointer: *mut dyn Any = self;
        match <dyn Any>::downcast_mut::<T>(self) {
            Some(unerased) => Ok(unerased),
            // SAFETY: Previous mutable borrow discarded if downcast_mut returns None
            None => Err(unsafe { &mut *pointer })
        }
    }
}