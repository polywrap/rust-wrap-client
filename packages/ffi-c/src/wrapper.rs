use std::{any::TypeId};

#[repr(C)]
pub struct WrapperHandle {
    pub type_id: TypeId,
    pub destroy: unsafe fn(*mut WrapperHandle),
    pub invoke: unsafe fn(*mut WrapperHandle, &[u8]) -> Result<usize, Error>,
    pub get_file: unsafe fn(*mut WrapperHandle, options: &GetFileOptions) -> Result<(), Error>,
}