use crate::align::align_marker::ValidAlignment;

#[derive(Default, Copy, Clone)]
pub struct Alignment<const ALIGN: usize> 
    where Self: ValidAlignment 
{
    marker: <Self as ValidAlignment>::Marker 
}

impl<const ALIGN: usize> Alignment<ALIGN>
    where Self: ValidAlignment 
{
    pub const fn new() -> Self {
        Self {
            marker: <Self as ValidAlignment>::MARKER
        }
    }
} 