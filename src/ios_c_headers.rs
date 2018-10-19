use worker::Worker;

use core_foundation::base::TCFType;
use core_foundation::string::{CFString, CFStringRef};
use libc::c_char;
use std::os::raw::c_void;

// A container for Rust structs
#[repr(C)]
pub struct Session {
    worker: *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn session_new() -> *mut Session {
    // Create a worker
    let mut worker = Worker::new();
    // Convert into a void pointer
    let worker_ptr: *mut c_void = &mut worker as *mut _ as *mut c_void;
    // Create a container
    let session = Session { worker: worker_ptr };
    // Return the raw pointer of the container on the heap
    Box::into_raw(Box::new(session))
}

#[no_mangle]
pub unsafe extern "C" fn session_action(session: *mut Session, string: *const c_char) -> CFStringRef {
    // Get input string as Rust String
    let string = cstring_to_str(&string).unwrap();
    // Get worker pointer as Rust pointer
    let worker_ptr = (*session).worker;
    let worker: &mut Worker = &mut *(worker_ptr as *mut Worker);
    // Call the `action` method
    let string = worker.action(string);
    // Create a Objective-C String
    let cf_string = CFString::new(&string);
    let cf_string_ref = cf_string.as_concrete_TypeRef();
    // Tell Rust to not manage this memory
    ::std::mem::forget(cf_string);

    return cf_string_ref;
}

// Useful conversion
unsafe fn cstring_to_str<'a>(cstring: &'a *const c_char) -> Option<&str> {
    if cstring.is_null() {
        return None;
    }
    let raw = ::std::ffi::CStr::from_ptr(*cstring);
    match raw.to_str() {
        Ok(s) => Some(s),
        Err(_) => None
    }
}

