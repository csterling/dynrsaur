use std::any::type_name;
use crate::erasure::TaggedErasure;

/// Default implementation of [downcast_unchecked](crate::erasure::Erasure::downcast_unchecked).
/// 
/// For use when implementing [Erasure](crate::erasure::Erasure) for types which implement
/// [TaggedErasure]. See [TaggedErasure] for why this can't be a blanket implementation.
/// 
/// Generic Parameters:
/// - `T`: the possible underlying type of the erasure.
/// - `E`: the type of the erasure.
pub fn defer_downcast_unchecked_to_try_downcast<
    T,
    E: TaggedErasure<T>
>(
    self_: E
) -> T {
    match self_.try_downcast() {
        Ok(unerased) => unerased,
        Err(_) => panic!("invalid downcast to '{}'", type_name::<T>())
    }
}
