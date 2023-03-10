use std::ffi::{CStr, c_char, CString};

pub fn get_string_from_cstr_ptr(str_ptr: *const c_char) -> String {
  let cstr = unsafe { CStr::from_ptr(str_ptr) };

  match cstr.to_str() {
    Ok(s) => s.to_string(),
    Err(e) => panic!("Error getting string from CStr: {}", e)
  }
}

pub fn instantiate_from_ptr<T>(ptr: *mut T) -> T {
  let data = unsafe {
    std::mem::forget(ptr);
    std::ptr::read(ptr)
  };
  data
}

pub fn instantiate_from_ptr_and_take_ownership<T>(ptr: *mut T) -> Box<T> {
  let boxed_data = unsafe { Box::from_raw(ptr) };
  boxed_data
}

pub fn into_raw_ptr_and_forget<T>(instance: T) -> *const std::ffi::c_void {
  let ptr = Box::into_raw(Box::new(instance)) as *const std::ffi::c_void;
  std::mem::forget(ptr);
  ptr
}

pub fn raw_ptr_from_str(str: &str) -> *const c_char {
  CString::new(str).unwrap().into_raw()
}
