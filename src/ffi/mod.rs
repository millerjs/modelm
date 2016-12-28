/// Target OS dependent ffi

pub mod types;

use std::sync::mpsc::Sender;
use self::types::KeyEvent;

// ======================================================================
// Import module according to target os

#[cfg(target_os = "macos")]
mod osx;

#[cfg(target_os = "linux")]
mod linux;

// ======================================================================
// Compile against os ffi module

#[cfg(target_os = "macos")]
pub fn register_listener(tx: &Sender<KeyEvent>) {
    self::osx::register_listener(tx);
}

#[cfg(target_os = "linux")]
pub fn register_listener(_: &Sender<KeyEvent>) {}


/// Start listener should never return
#[allow(unused_variables)]
pub fn start_listener(tx: &Sender<KeyEvent>) {
    #[cfg(target_os = "macos")]
    self::osx::start_listener();
    #[cfg(target_os = "linux")]
    self::linux::start_listener(tx)
}
