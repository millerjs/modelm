#![crate_name = "modelm"]

#[macro_use]
extern crate quick_error;
#[macro_use]
extern crate log;

extern crate env_logger;
extern crate ears;
extern crate libc;
extern crate rand;
extern crate regex;
extern crate yaml_rust;

#[macro_use]
pub mod macros;
pub mod keyboard;
pub mod ffi;
pub mod switch;
pub mod errors;

pub static VERSION: &'static str = "0.3.0";

static DEFAULT_SOUND_FILE_REGEX: &'static str = r"\.(wav|mp3)";
