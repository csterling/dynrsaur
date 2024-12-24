
mod auto_drop_unique;
pub use auto_drop_unique::AutoDropUnique;

mod erased;
pub use erased::Erased;

mod erased_box;
pub use erased_box::ErasedBox;

mod erasure;
pub use erasure::Erasure;

mod tagged_erasure;
mod inline_erasure;

pub use tagged_erasure::TaggedErasure;
