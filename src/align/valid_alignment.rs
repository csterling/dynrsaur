use std::fmt::Debug;
use std::panic::{RefUnwindSafe, UnwindSafe};
use sealed::sealed;

/// Indicates that the type represents a valid alignment.
/// 
/// Sealed trait that is only implemented for the [alignment marker types](crate::align::align_markers),
/// e.g. [`AlignMarker32`](crate::align::AlignMarker32), and [`Alignment<ALIGN>`](crate::align::Alignment).
#[sealed(pub(in super))]
pub trait ValidAlignment {
    /// Const access to the alignment marker for this alignment.
    /// 
    /// As the marker type is a ZST, there is only one possible value. This is that value.
    const MARKER: Self::Marker;
    
    /// The ZST which marks this alignment.
    type Marker: Default + Copy + Debug
        // We require all the auto-traits as bounds to convince Alignment<ALIGN> that it
        // can auto-inherit them. TODO: Include Freeze auto-trait when on nightly.
        + RefUnwindSafe + Send + Sync + Unpin + UnwindSafe;
}
