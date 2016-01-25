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

extern crate ears;
extern crate rand;

use rand::distributions::IndependentSample;
use rand::distributions::Range;

use ears::Sound;
use ears::AudioController;

use std::fs::read_dir;
use std::thread;
use std::sync::mpsc::channel;

use ffi::CFRunLoopRun;
use ffi::EventType;
use ffi::register_event_tap;

/// A composable Keyboard representation
#[repr(C)]
pub struct Keyboard {
    sounds: Vec<Sound>,
}


impl Keyboard {

    /// Default constructor for ClickHandler
    ///
    /// Create a new struct and associate a path to it.
    pub fn new(path: &str) -> Keyboard {
        let mut sounds = vec![];

        for path in read_dir(path).unwrap() {
            let p = path.unwrap().path().to_str().unwrap().to_string();
            let mut sound = Keyboard::load_sound(p);
            sound.set_relative(true);
            sounds.push(sound);
        }

        Keyboard {
            sounds: sounds,
        }
    }

    /// Sets the volume of the keyboard.
    ///
    /// Volume should be a decimal between 0 and 1
    pub fn set_volume(mut self, volume: f32) -> Keyboard {
        for sound in self.sounds.iter_mut() {
            sound.set_volume(volume);
        }
        self
    }

    /// Play a random key sound
    ///
    /// # Example
    /// ```ignore
    /// Keyboard::new("resources/modelm").click();
    /// ```
    pub fn click(&mut self) {
        let range = Range::new(0, self.sounds.len());
        let idx = range.ind_sample(&mut rand::thread_rng());
        let sound = &mut self.sounds[idx];
        sound.set_position([0.0, 0.0, 1.0]);
        sound.play();
    }

    /// Play a random key sound from position in 3D space.
    ///
    /// # Example
    /// ```ignore
    /// Keyboard::new("resources/modelm").click_pos([1.0, 0.0, 0.0]);
    /// ```
    pub fn click_pos(&mut self, position: [f32; 3]) {
        let range = Range::new(0, self.sounds.len());
        let idx = range.ind_sample(&mut rand::thread_rng());
        let sound = &mut self.sounds[idx];
        sound.set_position(position);
        sound.play();
    }

    /// Wraps around Sound::new()
    ///
    /// Load the sound located at the provided path.
    ///
    /// # Argument
    /// `path` - The path the the sound file
    ///
    /// # Return
    /// A Sound, panic if the sound file could not be read.
    ///
    /// # Example
    /// ```ignore
    /// use modelm::keyboard::Keyboard;
    /// let sound = Keyboard::load_sound("resources/modelm/1_.wav".to_string());
    /// ```
    fn load_sound(path: String) -> Sound {
        Sound::new(&*path).expect(&*format!("Could not load {:}.", path))
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
        thread::spawn(move || {
            register_event_tap(&tx);
            println!("Running event listener...\nPress ^C to exit.");
            CFRunLoopRun();
        });

        loop {
            let event = rx.recv().unwrap();
            let position = (25.0 - event.code as f32) / - 100.0;
            match event.etype {
                EventType::KeyUp => self.click_pos([position, 0.0, 1.0]),
                _ => ()
            }
        }
    }
}


#[cfg(test)]
mod test {
    #![allow(non_snake_case)]

    use super::Keyboard;

    #[test]
    fn keyboard_create_OK() -> () {
        Keyboard::new("resources/modelm");
    }
}
