//! Representation of keyboard switches
//!
//! This module contains a SwitchSound struct that holds options for
//! keyboard switches in relation to sounds.

use rand;
use ears::AudioController;
use ears::Sound;
use ffi::{KeyEvent, KeyCode, EventType};
use regex::Regex;
use rand::distributions::IndependentSample;
use rand::distributions::Range;
use keyboard::KeyboardOptions;
use std::path::Path;
use yaml_rust;
use yaml_rust::Yaml;
use ::errors::KeyboardError;

pub struct SwitchSound {
    sound: Sound,
    name: String,
}


pub struct Switch {
    pub sounds_keydown: Vec<SwitchSound>,
    pub sounds_keyup: Vec<SwitchSound>,
    pub keycode_regex: Regex,
    pub position: [f32; 3],
}


impl SwitchSound {
    fn from_path(path: &Path) -> Result<SwitchSound, KeyboardError>
    {
        let sound = try!(path.to_str()
            .and_then(|p| Sound::new(p))
            .ok_or(format!("Unable to load sound: {:?}", path)));

        let name = try!(path.file_name()
            .ok_or(format!("Unable to parse filename: {:?}", path))
            .and_then(|n| Ok(n.to_string_lossy().to_string())));

        Ok(SwitchSound {
            name: name,
            sound: sound,
        })
    }
}

macro_rules! play_random_sound {
    ($sounds: expr, $position: expr, $options: expr) => {
        {
            if $sounds.len() > 0 {
                let range = Range::new(0, $sounds.len());
                let idx = range.ind_sample(&mut rand::thread_rng());
                let sound = &mut $sounds[idx];
                sound.sound.set_position($position);
                sound.sound.set_volume($options.volume);
                debug!("Playing {}", sound.name);
                sound.sound.play();
            }
        }
    };
}


impl Switch {
    pub fn new() -> Switch
    {
        Switch {
            sounds_keydown: vec![],
            sounds_keyup: vec![],
            keycode_regex: Regex::new(".*").unwrap(),
            position: [0.0, 0.0, 1.0],
        }
    }

    pub fn with_keycode_regex(mut self, regex: Regex) -> Switch
    {
        self.keycode_regex = regex;
        self
    }

    pub fn load_sound_keydown(mut self, path: &Path) -> Result<Switch, KeyboardError>
    {
        self.sounds_keydown.push(try!(SwitchSound::from_path(path)));
        Ok(self)
    }

    pub fn load_sound_keyup(mut self, path: &Path) -> Result<Switch, KeyboardError>
    {
        self.sounds_keyup.push(try!(SwitchSound::from_path(path)));
        Ok(self)
    }

    pub fn handle_event(&mut self, event: KeyEvent, options: &KeyboardOptions) {
        let position = - (25.0 - event.code as f32) * options.x_scale / 300.0;
        match event.etype {
            EventType::KeyDown => {
                play_random_sound!(self.sounds_keydown, [position, 0.0, 1.0], options);
            },
            EventType::KeyUp => {
                play_random_sound!(self.sounds_keyup, [position, 0.0, 1.0], options);
            },
            _ => (),
        }
    }

    pub fn handles(&self, code: KeyCode) -> bool {
        self.keycode_regex.is_match(&*format!("{}", code))
    }

    pub fn from_yaml(yaml: &yaml_rust::Yaml) -> Result<Switch, KeyboardError>
    {
        let hash = try_yaml!(*yaml, Yaml::Hash, "switch must be a Hash [switch]");

        let regex_str = try_yaml!(yaml["keycode_regex"], Yaml::String,
                                  "config must have Hash [switch.keycode_regex]");

        info!("Parsed keycode_regex : {}", regex_str);

        let mut switch = Switch::new().with_keycode_regex(try!(Regex::new(&*regex_str)));

        if hash.contains_key(&Yaml::String("keydown_paths".into())){
            let keydown_paths = try_yaml!(yaml["keydown_paths"], Yaml::Array,
                                          "config must have Array [switch.keydown_paths]");

            for keydown_path in keydown_paths {
                let path = try!(keydown_path.as_str()
                    .ok_or(format!("Unable to parse path: {:?}", keydown_path)));
                info!("Parsed keydown path: {}", path);
                switch = try!(switch.load_sound_keydown(&Path::new(path)));
            }
        }

        if hash.contains_key(&Yaml::String("keyup_paths".into())){
            let keyup_paths = try_yaml!(yaml["keyup_paths"], Yaml::Array,
                                        "config must have Array [switch.keyup_paths]");

            for keyup_path in keyup_paths {
                let path = try!(keyup_path.as_str()
                    .ok_or(format!("Unable to parse path: {:?}", keyup_path)));
                info!("Parsed keyup path: {}", path);
                switch = try!(switch.load_sound_keyup(&Path::new(path)));
            }
        }
        Ok(switch)
    }
}
