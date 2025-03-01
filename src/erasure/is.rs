use std::any::type_name;
use std::marker::PhantomData;
use crate::erasure::{defer_downcast_unchecked_to_try_downcast, Erasure, TaggedErasure};

/// Context-type asserting that the given erasure *is* a `T`.
/// 
/// Generic Parameters:
/// - `T`: the underlying type of the erasure.
/// - `E`: the type of the erasure.
pub struct Is<T, E: TaggedErasure<T>> {
    /// The erasure that was checked, and is in fact underlying a `T`.
    erasure: E,
    /// Marker indicating that we indeed own a `T`.
    _marker: PhantomData<T>
}

impl<T, E: TaggedErasure<T>> Is<T, E> {
    /// Checks if the given [erasure][TaggedErasure] is an erased-`T`.
    /// 
    /// Returns:
    /// - `Ok(Self)`: if the erasure is an erased-`T`.
    /// - `Err(erasure)`: if the erasure is not an erased-`T`.
    pub fn new(erasure: E) -> Result<Self, E> {
        if erasure.is() {
            Ok(Self { erasure, _marker: PhantomData })
        } else {
            Err(erasure)
        }
    }
    
    /// Reclaims the erasure from the context.
    pub fn into_inner(self) -> E {
        self.erasure
    }
    
    /// Reclaims the underlying type from the erasure.
    pub fn downcast(self) -> T {
        match self.erasure.try_downcast() {
            Ok(downcast) => downcast,
            Err(_) => panic!(
                "inconsistent TaggedErasure impl: {} claims to be a {} but isn't",
                type_name::<E>(),
                type_name::<T>()
            )
        }
    }
}

impl<T, E: TaggedErasure<T>> Erasure<T> for Is<T, E> {
    unsafe fn downcast_unchecked(self) -> T {
        defer_downcast_unchecked_to_try_downcast(self)
    }
}

impl<T, E: TaggedErasure<T>> TaggedErasure<T> for Is<T, E> {
    fn is(&self) -> bool {
        true
    }

    fn try_downcast(self) -> Result<T, Self> {
        Ok(self.downcast())
    }
}

#[cfg(test)]
mod tests {
    use std::any::Any;
    use std::ops::Deref;
    use crate::erasure::Is;

    #[test]
    fn is() {
        const STRING: &str = "this is a string";
        
        let erased_string: Box<dyn Any> = Box::new(String::from(STRING));
        
        let is = Is::<&String, _>::new(erased_string.deref())
            .expect("erased_string erases a String");
        
        assert_eq!(is.downcast(), STRING);
    }
}