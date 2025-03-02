//! Tools for working with alignment.
//! 
//! Contains:
//! - [ZSTs](align_markers) with specified alignments (e.g. [AlignMarker32](align_markers::AlignMarker32)),
//! - a [const-generic equivalent](Alignment) for use in generic contexts,
//! - the [ValidAlignment] trait for use in trait bounds w.r.t. alignment, and,
//! - [AlignedBytes], an extension of `[u8; SIZE]` which also can specify alignment with
//!   a const-generic parameter `ALIGN`.

pub mod align_markers;

mod aligned_bytes;
pub use aligned_bytes::AlignedBytes;

mod alignment;
pub use alignment::Alignment;

mod valid_alignment;
pub use valid_alignment::ValidAlignment;
