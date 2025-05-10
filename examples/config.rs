use colorutil::config::{load_config, ConfigBase};
use std::env::set_current_dir;

fn main() {
    set_current_dir(format!("{}/examples", env!("CARGO_MANIFEST_DIR"))).unwrap();

    let toml = load_config::<ConfigBase>("config").unwrap();
    
    let config = toml.parse().unwrap();
    
    println!("{:?}", config);
    println!();
    println!("{:?}", config.palettes[&config.palette]);
}
