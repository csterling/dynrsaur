use std::marker::PhantomData;
use crate::erasure::auto_drop_unique::AutoDropUnique;
use crate::erasure::{Erase, Erasure};

/// An owned, type-erased value on the heap, which preserves the lifetime of the value it was
/// created from.
/// 
/// Generic Parameters:
/// - `'lifetime`: the lifetime of the `ErasedBox`, which ties it to the originating value.
/// - `E`: the inderlying (lifetime-less) [erasure](Erasure).
pub struct ErasedBox<'lifetime, E: 'lifetime = AutoDropUnique> {
    pointer: E,
    lifetime: PhantomData<&'lifetime ()>,
}

impl<'lifetime, E: 'lifetime> ErasedBox<'lifetime, E> {
    /// Takes ownership of `value` and [erases](Erase) its type.
    pub fn new<T: 'lifetime>(value: T) -> Self where E: Erase<T> {
        Self {
            pointer: E::erase(value),
            lifetime: PhantomData
        }
    }
}

impl<
    'lifetime,
    T: 'lifetime,
    E: Erasure<T> + 'lifetime
> Erasure<T> for ErasedBox<'lifetime, E> {
    unsafe fn downcast_unchecked(self) -> T {
        self.pointer.downcast_unchecked()
    }
}

impl<
    'borrow,
    'lifetime: 'borrow,
    T: 'lifetime,
    E: 'lifetime
> Erasure<&'borrow T> for &'borrow ErasedBox<'lifetime, E> 
    where &'borrow E: Erasure<&'borrow T>
{
    unsafe fn downcast_unchecked(self) -> &'borrow T {
        <&'borrow E as Erasure<&'borrow T>>::downcast_unchecked(&self.pointer)
    }
}

impl<
    'borrow,
    'lifetime: 'borrow,
    T: 'lifetime,
    E: 'lifetime 
> Erasure<&'borrow mut T> for &'borrow mut ErasedBox<'lifetime, E> 
    where &'borrow mut E: Erasure<&'borrow mut T>
{
    unsafe fn downcast_unchecked(self) -> &'borrow mut T {
        <&'borrow mut E as Erasure<&'borrow mut T>>::downcast_unchecked(&mut self.pointer)
    }
}
