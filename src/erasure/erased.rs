/// Type for use in generics to indicate that the actual type has been erased. Empty type so that
/// methods which use the generic type then become inaccessible.
pub enum Erased {}
