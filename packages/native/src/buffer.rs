use crate::utils::{Buffer, into_raw_ptr_and_forget};

pub extern "C" fn create_buffer(ptr: *mut u8, length: usize) -> *const Buffer {
  let buffer = Buffer {
    data: ptr,
    len: length
  };

  into_raw_ptr_and_forget(buffer) as *const _
}