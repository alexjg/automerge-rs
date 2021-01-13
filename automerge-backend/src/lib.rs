extern crate fxhash;
extern crate hex;
//extern crate im_rc;
extern crate maplit;
extern crate rand;
extern crate uuid;
extern crate web_sys;

#[allow(unused_macros)]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

mod actor_map;
mod backend;
mod change;
mod columnar;
mod concurrent_operations;
mod encoding;
mod error;
mod internal;
mod object_store;
mod op_handle;
mod op_set;
mod ordered_set;
mod pending_diff;
mod time;

pub use backend::Backend;
pub use change::Change;
pub use error::AutomergeError;
