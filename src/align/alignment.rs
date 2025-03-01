use crate::align::valid_alignment::ValidAlignment;

/// Const-generic alignment marker.
/// 
/// Generic Parameters:
/// - `ALIGN`: the required alignment of the type. Must be a [valid alignment](ValidAlignment).
#[derive(Default, Copy, Clone, Debug)]
pub struct Alignment<const ALIGN: usize> 
    where Self: ValidAlignment
{
    _marker: <Self as ValidAlignment>::Marker
}

impl<const ALIGN: usize> Alignment<ALIGN>
    where Self: ValidAlignment 
{
    pub const fn new() -> Self {
        Self {
            _marker: <Self as ValidAlignment>::MARKER
        }
    }
} 