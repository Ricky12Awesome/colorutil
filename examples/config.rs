use colorutil::config::{load_config, Config};
use std::env::set_current_dir;

fn main() {
    set_current_dir(format!("{}/examples", env!("CARGO_MANIFEST_DIR"))).unwrap();
    
    let toml = load_config::<Config>("config").unwrap();

    println!("{:?}", toml.prefix);
}
