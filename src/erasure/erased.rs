/// Type for use in generics to indicate that the actual type has been erased.
///
/// Empty type so that methods which use the generic type then become inaccessible.
pub enum Erased {}

#[cfg(test)]
mod tests {
    use crate::erasure::Erased;

    #[test]
    fn erased_is_zst() {
        assert_eq!(size_of::<Erased>(), 0);
    }
    
    #[test]
    fn erased_is_align_one() {
        assert_eq!(align_of::<Erased>(), 1);
    }
}