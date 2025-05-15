use colorutil::config::{load_config, ConfigBase};
use std::env::set_current_dir;
use colorutil::color::parse_format;

fn main() {
    set_current_dir(format!("{}/examples", env!("CARGO_MANIFEST_DIR"))).unwrap();

    let toml = load_config::<ConfigBase>("config").unwrap();
    
    let config = toml.parse().unwrap();

    let p =  &config.palettes[&config.palette];
    let c =  &p["r2"];
    // println!("{:?}", config);
    println!();
    println!("{:?}", config.palettes[&config.palette]["r2"]);
    println!("{:?}", parse_format(c, "hex", p));
}
