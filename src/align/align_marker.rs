

pub trait ValidAlignment {
    const MARKER: Self::Marker;
    
    type Marker: Default + Copy;
}

macro_rules! align_marker {
    ($align:literal) => {
        ::paste::paste!{
            #[derive(Default, Copy, Clone)]
            #[repr(align($align))]
            pub struct [<AlignMarker $align>];
            
            impl crate::align::align_marker::ValidAlignment for crate::align::alignment::Alignment<$align>  {
                const MARKER: Self::Marker = [<AlignMarker $align>];
                type Marker = [<AlignMarker $align>];
            }
        }
    };
}

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

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
mod target_pointer_width_32 {
    align_marker!(65536);
    // TODO: the rest
}
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
pub use target_pointer_width_32::*;

#[cfg(any(target_pointer_width = "64"))]
mod target_pointer_width_64 {
    // TODO: ZSTs
}
#[cfg(any(target_pointer_width = "64"))]
pub use target_pointer_width_64::*;
