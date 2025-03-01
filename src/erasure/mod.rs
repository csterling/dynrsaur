//! Tools for creating erased types in dynamic contexts.
//! 
//! ## [`AutoDropUnique`] vs. [`Box<dyn Unknown>`](Unknown)
//! 
//! These 2 types fulfil the same purpose, that is, being [untagged erasures](Erasure) of any 
//! ([sized](Sized)) type, held on the heap. The difference is in their layout. `AutoDropUnique` is
//! 1 word in size vs. `Box<dyn Unknown>`'s 2 words. However, `AutoDropUnique` keeps the
//! [drop](Drop::drop) implementation with the value on the heap, so the heap allocation is bigger
//! (how much bigger depends on the size/alignment of the underlying value).

mod auto_drop_unique;
pub use auto_drop_unique::AutoDropUnique;

mod defer_downcast_unchecked_to_try_downcast;
pub use defer_downcast_unchecked_to_try_downcast::defer_downcast_unchecked_to_try_downcast;

mod erase;
pub use erase::Erase;

mod erased;
pub use erased::Erased;

mod erased_box;
pub use erased_box::ErasedBox;

mod erasure;
pub use erasure::Erasure;

mod inline_erasure;
pub use inline_erasure::InlineErasure;

mod is;
pub use is::Is;

mod tagged_erasure;
pub use tagged_erasure::TaggedErasure;

mod unknown;
pub use unknown::Unknown;
