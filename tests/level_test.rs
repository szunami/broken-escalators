extern crate broken_escalators;

use amethyst::prelude::Config;
use broken_escalators::levels::LevelConfig;
use std::fs;
#[test]
fn all_levels_deser() {
    let level_paths = fs::read_dir("assets/levels").unwrap();
    for path in level_paths {
        let actual_path = path.unwrap().path();
        let load_result: Result<_, _> = LevelConfig::load(actual_path.as_path());
        assert!(!load_result.is_err());
    }
}
