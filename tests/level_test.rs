use std::fs;
use crate broken_escalators;
use broken_escalators::levels::LevelConfig;
#[test]
fn all_levels_deser() {
    let level_paths = fs::read_dir("assets/levels").unwrap();
    for path in level_paths {
        println!("Name: {}", path.unwrap().path().display());
        LevelConfig::load(path.unwrap());
    }
}