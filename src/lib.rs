#[cfg(target_os = "ios")]
extern crate core_foundation;
#[cfg(target_os = "ios")]
mod ios_c_headers;
#[cfg(target_os = "ios")]
extern crate libc;
#[cfg(target_os = "ios")]
pub use ios_c_headers::*;


#[cfg(target_os = "android")]
mod android_c_headers;
#[cfg(target_os = "android")]
pub mod java_glue;
#[cfg(target_os = "android")]
#[macro_use]
extern crate log;
#[cfg(target_os = "android")]
extern crate log_panics;

/*cascade! {
    if #[cfg(target_os = "ios")] { 

    } else if #[cfg(target_os = "android")] { 

    } else { }
}*/

pub mod worker;
