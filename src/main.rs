//! Small OSX commandline app that uses your keyboard to
//! emulate mechanical keyboard audio.

#[macro_use]
extern crate log;
extern crate modelm;
extern crate ears;
extern crate env_logger;
extern crate clap;

use clap::{Arg, App, ArgMatches};
use modelm::keyboard::Keyboard;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

static DEFAULT_PATH: &'static str = "resources/modelm";

/// Setup logging (cli arg overwrites env var for dtt crate)
pub fn setup_logging(matches: &ArgMatches)
{
    let rust_log = env::var("RUST_LOG").unwrap_or("".to_owned());

    let log_level = match matches.is_present("DEBUG") {
        false => "modelm=info",
        true => "modelm=debug",
    };

    env::set_var("RUST_LOG", &*format!("{},{}", rust_log, log_level));
    env_logger::init().unwrap();

    debug!("Set log level to {}", log_level);
}


fn main() -> () {
    ears::init();

    let matches = App::new("modelm")
        .version("0.2.0")
        .author("Joshua Miller <jsmiller@uchicago.edu>")
        .about("Turns your computer into a mechanical keyboard emulator!")
        .arg(Arg::with_name("VOLUME")
             .short("v")
             .long("volume")
             .help("Adjust the keyboard volume in range [0.0, 1.0]")
             .takes_value(true))
        .arg(Arg::with_name("CLICKS")
             .short("c")
             .long("click-directory")
             .help("Specify the directory to load click sounds from")
             .takes_value(true))
        .arg(Arg::with_name("CONFIG")
             .long("config")
             .help("Specify the config to parse click options from")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("DEBUG")
             .short("d")
             .long("debug")
             .takes_value(false)
             .help("Debug output"))
        .arg(Arg::with_name("XSCALE")
             .short("x")
             .long("x-scale")
             .help("Specify the pan amount for the positional sound of clicks. \
                    A decimal (default: 1.0).  The larger the value, the further \
                    apart the clicks will sound. A value of 0 turns off positional \
                    sound. A value < 0 reverses the directionality.")
             .takes_value(true))
        .get_matches();

    setup_logging(&matches);

    // working directory
    let dir = matches.value_of("CLICKS").unwrap_or(DEFAULT_PATH);
    env::set_current_dir(&Path::new(&*dir)).expect(&*format!("Unable to work in dir {}", dir));

    // config path
    let config_path = matches.value_of("CONFIG").unwrap();

    // volume
    let volume: f32 = matches.value_of("VOLUME").unwrap_or("1.0").parse()
        .expect("Volume must be a decimal between 0 and 1.");

    // x-scale
    let x_scale: f32 = matches.value_of("XSCALE").unwrap_or("1.0").parse()
        .expect("x-scale must be a decimal. (default: 1.0)");

    // Read the config file
    let mut config = String::new();
    let mut config_file = File::open(&config_path)
        .expect(&*format!("unable to open: {}", config_path));
    config_file.read_to_string(&mut config)
        .expect(&*format!("unable to read: {}", config_path));

    // Create a keyboard
    let keyboard = Keyboard::new()
        .set_volume(volume)
        .set_x_scale(x_scale)
        .load_config_yaml(&*config);

    // Run the keyboard
    match keyboard {
        Ok(mut keyboard) => keyboard.listen(),
        Err(error) => error!("Unable to initialize keyboard: {:?}", error),
    };
}
