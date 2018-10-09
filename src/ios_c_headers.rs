/// Define a structure that wraps the rust `Session` type with FFI

// FIXME:
// the `Session` is the android wrapper. move it to a sep file
// create a iOS wrapper
// create the actual type (session?) somewhere else
// maybe find a nice thing to do, maybe even with strings
// for iOS, consider using rust's NSString support to return a proper memory-managed string
// instead of relying on cstring hell
use worker::Worker;

use core_foundation::base::TCFType;
use core_foundation::string::{CFString, CFStringRef};
use libc::c_char;
use std::ffi::CStr;
use std::os::raw::c_void;
use std::ffi::CString;
use std::mem;

unsafe fn cstring_to_str<'a>(cstring: &'a *const c_char) -> Option<&str> {
    if cstring.is_null() {
        return None;
    }
    let raw = CStr::from_ptr(*cstring);
    match raw.to_str() {
        Ok(s) => Some(s),
        Err(_) => None
    }
}

struct State {
    last_string: Option<CString>
}

#[repr(C)]
pub struct Session {
    worker: *mut c_void,
    state: *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn session_new() -> *mut Session {
    let mut worker = Worker::new();
    let mut state = State { last_string: None };
    let worker_ptr: *mut c_void = &mut worker as *mut _ as *mut c_void;
    let state_ptr: *mut c_void = &mut state as *mut _ as *mut c_void;
    let session = Session { worker: worker_ptr, state: state_ptr };
    Box::into_raw(Box::new(session))
}

#[no_mangle]
pub unsafe extern "C" fn session_action(session: *mut Session, string: *const c_char) -> CFStringRef {
    let string = cstring_to_str(&string).unwrap();
    let worker_ptr = (*session).worker;
    let worker: &mut Worker = &mut *(worker_ptr as *mut Worker);
    CFString::new(&worker.action(string)).as_concrete_TypeRef()
}

#[no_mangle]
pub unsafe extern "C" fn session_action2(session: *mut Session, string: *const c_char) -> *const c_char {
    let string = cstring_to_str(&string).unwrap();
    let worker_ptr = (*session).worker;
    let worker: &mut Worker = &mut *(worker_ptr as *mut Worker);

    let state_ptr = (*session).state;
    let state: &mut State = &mut *(state_ptr as *mut State);

    let str = CString::new(worker.action(string)).unwrap();
    state.last_string = Some(str);
    //let ptr = str.as_ptr();
    // this crashes.
    // cleanup would probably be best..
    // or getting fucking cfstring to work.
    let ptr = &state.last_string.as_ref().unwrap().as_ptr();
    // This leaks!
    //mem::forget(str);
    *ptr
}
