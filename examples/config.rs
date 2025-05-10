use colorutil::config::Config;
use std::env::set_current_dir;

fn main() {
    set_current_dir(format!("{}/examples", env!("CARGO_MANIFEST_DIR"))).unwrap();

    let file = std::fs::read_to_string("config.toml").unwrap();
    let toml = toml::from_str::<Config>(&file).unwrap();

    println!("{:?}", toml);
}
