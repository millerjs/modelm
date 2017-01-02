/// Target OS dependent ffi

pub mod types;

use self::types::KeyEvent;
use std::sync::mpsc::Sender;


// ======================================================================
// Import module according to target os

#[cfg(target_os = "macos")] mod osx;
#[cfg(target_os = "linux")] mod linux;


// ======================================================================
// Compile against os ffi module

/// Register a listener.
///    - On Linux, this is a no-op because we rely on an input device
///      that is already listening.
///    - On OSX, this registers a Quartz Event Tap
#[allow(unused_variables)]
pub fn register_listener(tx: &Sender<KeyEvent>) {
    #[cfg(target_os = "macos")] self::osx::register_listener(tx);
}

/// Start listener should never return.  The listener will send events
/// via the channel `tx`. (On OSX, `tx` should already have been
/// passed to a registered event tap by calling `register_lisener` and
/// will not be used here.)
#[allow(unused_variables)]
pub fn start_listener(tx: &Sender<KeyEvent>) {
    #[cfg(target_os = "macos")] self::osx::start_listener();
    #[cfg(target_os = "linux")] self::linux::start_listener(tx)
}
