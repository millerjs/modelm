/// Linux key handler ffi

use std::fs::File;
use std::io::prelude::*;
use std::sync::mpsc::Sender;
use std::{mem, slice, io, env};
use super::types::{EventType, KeyEvent};

type LinuxEventCode = u16;
type LinuxEventType = u16;
type LinuxEventValue = u32;
type LinuxKernelTime = u64;

#[repr(C)]
#[derive(Debug)]
struct TimeValue {
   	__kernel_time_t: LinuxKernelTime,
	__kernel_suseconds_t: LinuxKernelTime,
}

#[repr(C)]
#[derive(Debug)]
struct InputEvent {
	time: TimeValue,
	etype: LinuxEventType,
	code: LinuxEventCode,
    value: LinuxEventValue,
}

/// Returns true if the code is considered a "flag", i.e. a modifier
/// key
fn is_flag(code: LinuxEventCode) -> bool {
    match code {
        100 | 125 | 29 | 42 | 54 | 56 | 58 | 97 => true,
        _ => false,
    }
}


impl Into<KeyEvent> for InputEvent {
    fn into(self) -> KeyEvent {
        KeyEvent {
            code: self.code,
            etype: match is_flag(self.code) {
                true => EventType::FlagsChanged,
                false => match self.value {
                    0 => EventType::KeyUp,
                    _ => EventType::KeyDown,
                },
            },
        }
    }
}


/// Reads event struct from input device
fn read_event(device: &mut File) -> Result<InputEvent, io::Error> {
    let mut event: InputEvent = unsafe { mem::zeroed() };
    let event_size = mem::size_of::<InputEvent>();

    unsafe {
        try!(device.read_exact(slice::from_raw_parts_mut(
            &mut event as *mut _ as *mut u8,
            event_size
        )));
    }

    if  event.etype != 1 {
        return read_event(device);
    }

    debug!("read input event {:?}", event);
    Ok(event)
}


/// Opens the input device, will use MODELM_INPUT_DEVICE environment
/// variable if set
fn open_device() -> Result<File, io::Error> {
    let path = env::var("MODELM_INPUT_DEVICE")
        .unwrap_or("/dev/input/event0".to_owned());
    File::open(path)
}


/// Sends KeyEvents to the channel. Returns only on error.
pub fn start_listener(channel: &Sender<KeyEvent>) {
    let mut device = open_device().expect("unable to open device");

    loop {
        let _ = read_event(&mut device)
            .map(|event| channel.send(event.into()))
            .map_err(|err| error!("Unable to parse event, {:}", err));
    }
}
