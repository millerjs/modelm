//! Keyboard emulation operations
//!
//! This module contains a Keyboard struct that emulates the sounds of
//! your favorite keyboard.
//!
//! The Keyboard currently uses the `ears` crate to play sounds with
//! OpenAL.
//!
//! # Example
//! ```ignore
//! ears::init();
//! Keyboard::new("resources/modelm").listen();
//! ```

use ::DEFAULT_SOUND_FILE_REGEX;
use ffi::{register_listener, start_listener};
use ffi::types::{EventType, KeyCode, KeyEvent};
use regex::Regex;
use std::collections::HashSet;
use std::fs::read_dir;
use std::sync::mpsc::channel;
use std::thread;
use switch::Switch;
use yaml_rust;
use yaml_rust::Yaml;
use ::errors::KeyboardError;

/// Keyboard representation
#[repr(C)]
pub struct Keyboard {
    switches: Vec<Switch>,
    sound_file_regex: Regex,
    keys_down: HashSet<KeyCode>,
    options: KeyboardOptions,
}


pub struct KeyboardOptions {
    pub x_scale: f32,
    pub volume: f32,
}


impl KeyboardOptions {
    pub fn default() -> Self
    {
        KeyboardOptions {
            x_scale: 1.0,
            volume: 1.0,
        }
    }
}

impl Keyboard {

    /// Default constructor for Keyboard
    ///
    /// Create a new Keyboard with default members
    pub fn new() -> Keyboard
    {
        Keyboard {
            keys_down: HashSet::new(),
            options: KeyboardOptions::default(),
            switches: vec![],
            sound_file_regex: Regex::new(DEFAULT_SOUND_FILE_REGEX).unwrap(),
        }
    }

    pub fn load_config_yaml(mut self, config: &str) -> Result<Keyboard, KeyboardError>
    {
        let parsed = try!(yaml_rust::YamlLoader::load_from_str(config));
        let yaml = &parsed[0];

        let switches = try_yaml!(yaml["switches"], Yaml::Array,
                                 "config must have Array [switches]");

        for switch_config in switches {
            self.switches.push(try!(Switch::from_yaml(&switch_config)));
        }

        Ok(self)
    }

    /// Adds a handler using all the sounds in the given directory
    ///
    /// # Argument
    /// `directory` - Path to read sound files from
    pub fn add_default_handler(mut self, directory: &str) -> Result<Keyboard, KeyboardError>
    {
        let mut switch = Switch::new();
        for path in read_dir(directory).unwrap() {
            let path = path.unwrap().path();
            if self.sound_file_regex.is_match(path.to_str().unwrap()) {
                switch = try!(switch.load_sound_keydown(&path));
            }
        }
        self.switches.push(switch);
        Ok(self)
    }

    /// Adds a user created handler
    ///
    /// # Argument
    /// `switch` - the Switch object to add
    pub fn switch(mut self, switch: Switch) -> Keyboard
    {
        self.switches.push(switch);
        self
    }

    /// Sets the volume of the keyboard.
    ///
    /// Volume should be a decimal between 0 and 1
    pub fn set_volume(mut self, volume: f32) -> Keyboard {
        self.options.volume = volume;
        self
    }

    /// Set the pan amount for the positional sound of clicks.
    ///
    /// A decimal (default: 1.0).  The larger the value, the further
    /// apart the clicks will sound. A value of 0 turns off positional
    /// sound. A value < 0 reverses the directionality.
    ///
    /// # Argument
    /// `x_scale` - Amount to scale sounds left and right.
    ///
    /// Scale should be a decimal.
    pub fn set_x_scale(mut self, x_scale: f32) -> Keyboard {
        self.options.x_scale = x_scale;
        self
    }

    /// Listener to play sound.
    ///
    /// Play a sound when the an event is added to the channel by the
    /// callback
    ///
    /// # Example
    /// ```ignore
    /// ears::init();
    /// Keyboard::new("resources");
    /// ```
    pub fn listen(&mut self) {
        let (tx, rx) = channel();

        // create listener thread
        thread::spawn(move || {
            register_listener(&tx);
            info!("Running event listener...");
            info!("Press ^C to exit.");
            start_listener(&tx);
        });

        // poll channel for events
        loop {
            match rx.recv() {
                Ok(event) => self.handle_event(event),
                Err(err) => return info!("Channel to listener closed, {:}", err),
            }
        }
    }

    /// Returns the index of the handler for a KeyCode
    ///
    /// # Argument
    /// `code` - The key code  of the event
    fn get_switch_index(&self, code: KeyCode) -> Option<usize> {
        for (i, switch) in self.switches.iter().enumerate() {
            if switch.handles(code) {
                return Some(i)
            }
        }
        None
    }

    /// Looks-up the handler for a KeyCode and calls the handler with
    /// the event.
    ///
    /// # Argument
    /// `event` - The instance of the event to handle
    pub fn call_event_handler(&mut self, event: KeyEvent) {
        match self.get_switch_index(event.code) {
            Some(i) => self.switches[i].handle_event(event, &self.options),
            None => (),
        };
    }

    /// Adjusts keyboard state given Event and calls a handler.
    ///
    /// # Argument
    /// `event` - The instance of the event to record and handle
    pub fn handle_event(&mut self, event: KeyEvent) {
        match event.etype {
            EventType::KeyDown => {
                if !self.keys_down.contains(&event.code) {
                    self.keys_down.insert(event.code);
                    self.call_event_handler(event);
                }
            },
            EventType::KeyUp => {
                if self.keys_down.contains(&event.code) {
                    self.keys_down.remove(&event.code);
                    self.call_event_handler(event);
                }
            },
            _ => ()
        };
    }
}


#[cfg(test)]
mod test {
    #![allow(non_snake_case)]

    use super::Keyboard;

    #[test]
    fn keyboard_create_OK() -> () {
        let _ = Keyboard::new().load_config_yaml(r"switches:
   -  keycode_regex: '49'
      keydown_paths:
        - spacebar.wav
      keyup_paths:
        - spacebar.wav
   -  keycode_regex: '\d+'
      keydown_paths:
        - 1_.wav
");
    }
}
