//!# A Minimal Cli Frontend
//!It calls chip8 lib. It accepts the game rom and optionally, the config file
//!```
//! ./ace_cli [game_location] [config_location]
//!```
//!

use std::env;

///The start of ace_cli
fn main() {
    let mut env_args = env::args();
    env_args.next();
    let rom: String = env_args.next().unwrap();
    let config: String = env_args
        .next()
        .unwrap_or_else(|| String::from("config/config.ini"));
    chip8::load(&rom, &config);
}
