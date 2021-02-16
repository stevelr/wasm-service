// build.rs
// Compile config.toml settings into src/config.rs

use config_struct::{Error, StructOptions};

fn main() -> Result<(), Error> {
    config_struct::create_struct("config.toml", "src/config.rs", &StructOptions::default())
}
