pub use crate::CBaseClass;

#[derive(Debug)]
#[repr(C)]
pub struct CBaseClassNonCopyable {
    pub c_base_class: CBaseClass,
}

impl CBaseClassNonCopyable {
}