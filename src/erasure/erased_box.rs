use std::marker::PhantomData;
use crate::erasure::auto_drop_unique::AutoDropUnique;
use crate::erasure::Erasure;

/// An owned, type-erased value on the heap, which remembers the lifetime of the value it was 
/// created from.
pub struct ErasedBox<'lifetime> {
    pointer: AutoDropUnique,
    lifetime: PhantomData<&'lifetime ()>,
}

impl<'lifetime> ErasedBox<'lifetime> {
    pub fn new<T: 'lifetime>(value: T) -> Self {
        Self {
            pointer: AutoDropUnique::new(value).erase(),
            lifetime: PhantomData
        }
    }
}

impl<'lifetime, T: 'lifetime> Erasure<T> for ErasedBox<'lifetime> {
    unsafe fn downcast(self) -> T {
        <AutoDropUnique as Erasure<T>>::downcast(self.pointer)
    }
}

impl<'borrow, 'lifetime: 'borrow, T: 'lifetime> Erasure<&'borrow mut T> for &'borrow mut ErasedBox<'lifetime> {
    unsafe fn downcast(self) -> &'borrow mut T {
        <&'borrow mut AutoDropUnique as Erasure<&'borrow mut T>>::downcast(&mut self.pointer)
    }
}

impl<'borrow, 'lifetime: 'borrow, T: 'lifetime> Erasure<&'borrow T> for &'borrow ErasedBox<'lifetime> {
    unsafe fn downcast(self) -> &'borrow T {
        <&'borrow AutoDropUnique as Erasure<&'borrow T>>::downcast(&self.pointer)
    }
}
