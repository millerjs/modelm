/// Define types to be passed between os and modelm

pub type KeyCode = u16;

#[derive(Debug)]
#[repr(C)]
pub enum EventType {
    KeyDown,
    KeyUp,
    FlagsChanged,
}

#[derive(Debug)]
#[repr(C)]
pub struct KeyEvent {
    pub etype: EventType,
    pub code: KeyCode,
}
