//! Tools for working with alignment.
//! 
//! Contains:
//! - ZSTs with specified alignments (e.g. [AlignMarker32]),
//! - a [const-generic equivalent](Alignment) for use in generic contexts,
//! - the [ValidAlignment] trait for use in trait bounds w.r.t. alignment, and,
//! - [AlignedBytes], an extension of `[u8; SIZE]` which also can specify alignment with
//!   a const-generic parameter `ALIGN`.

mod align_marker;
pub use align_marker::*;

mod aligned_bytes;
pub use aligned_bytes::AlignedBytes;

mod alignment;
pub use alignment::Alignment;

mod valid_alignment;
pub use valid_alignment::ValidAlignment;
