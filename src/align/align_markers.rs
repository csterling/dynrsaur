//! ZSTs with specified alignment.

use sealed::sealed;

/// Generates a new ZST with the prescribed alignment.
macro_rules! align_marker {
    ($align:literal) => {
        ::paste::paste!{
            #[doc = concat!("ZST with an alignment of ", stringify!($align), " bytes.")]
            #[derive(Default, Copy, Clone, Debug)]
            #[repr(align($align))]
            pub struct [<AlignMarker $align>];
            
            #[sealed]
            impl crate::align::valid_alignment::ValidAlignment for [<AlignMarker $align>]  {
                const MARKER: Self::Marker = [<AlignMarker $align>];
                type Marker = [<AlignMarker $align>];
            }
            
            #[sealed]
            impl crate::align::valid_alignment::ValidAlignment for crate::align::alignment::Alignment<$align>  {
                const MARKER: Self::Marker = [<AlignMarker $align>];
                type Marker = [<AlignMarker $align>];
            }
        }
    };
}

// FIXME: Restrict higher alignments on 16-bit platforms
align_marker!(1);
align_marker!(2);
align_marker!(4);
align_marker!(8);
align_marker!(16);
align_marker!(32);
align_marker!(64);
align_marker!(128);
align_marker!(256);
align_marker!(512);
align_marker!(1024);
align_marker!(2048);
align_marker!(4096);
align_marker!(8192);
align_marker!(16384);
align_marker!(32768);
align_marker!(65536);
align_marker!(131072);
align_marker!(262144);
align_marker!(524288);
align_marker!(1048576);
align_marker!(2097152);
align_marker!(4194304);
align_marker!(8388608);
align_marker!(16777216);
align_marker!(33554432);
align_marker!(67108864);
align_marker!(134217728);
align_marker!(268435456);
align_marker!(536870912);
