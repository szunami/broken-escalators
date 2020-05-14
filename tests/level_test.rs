extern crate broken_escalator;

use std::fs;
use broken_escalator::levels::LevelConfig;
use amethyst::prelude::Config;
#[test]
fn all_levels_deser() {
    let level_paths = fs::read_dir("assets/levels").unwrap();
    for path in level_paths {
        let x = path.unwrap();
        let y = x.path();
        let z = y.as_path();
        println!("{}", z.display());
        let load_result: Result<_, _> = LevelConfig::load(z);
        println!("{:?}", load_result);
        assert!(!load_result.is_err());
    }
}
