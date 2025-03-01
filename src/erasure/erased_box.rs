use std::marker::PhantomData;
use crate::erasure::auto_drop_unique::AutoDropUnique;
use crate::erasure::{Erase, Erasure};

/// An owned, type-erased value on the heap, which preserves the lifetime of the value it was
/// created from.
/// 
/// Generic Parameters:
/// - `'lifetime`: the lifetime of the `ErasedBox`, which ties it to the originating value.
/// - `E`: 
pub struct ErasedBox<'lifetime, E = AutoDropUnique> {
    pointer: E,
    lifetime: PhantomData<&'lifetime ()>,
}

impl<'lifetime, E> ErasedBox<'lifetime, E> {
    pub fn new<T: 'lifetime>(value: T) -> Self where E: Erase<T> {
        Self {
            pointer: E::erase(value),
            lifetime: PhantomData
        }
    }
}

impl<'lifetime, T: 'lifetime> Erasure<T> for ErasedBox<'lifetime> {
    unsafe fn downcast_unchecked(self) -> T {
        <AutoDropUnique as Erasure<T>>::downcast_unchecked(self.pointer)
    }
}

impl<'borrow, 'lifetime: 'borrow, T: 'lifetime> Erasure<&'borrow mut T> for &'borrow mut ErasedBox<'lifetime> {
    unsafe fn downcast_unchecked(self) -> &'borrow mut T {
        <&'borrow mut AutoDropUnique as Erasure<&'borrow mut T>>::downcast_unchecked(&mut self.pointer)
    }
}

impl<'borrow, 'lifetime: 'borrow, T: 'lifetime> Erasure<&'borrow T> for &'borrow ErasedBox<'lifetime> {
    unsafe fn downcast_unchecked(self) -> &'borrow T {
        <&'borrow AutoDropUnique as Erasure<&'borrow T>>::downcast_unchecked(&self.pointer)
    }
}
