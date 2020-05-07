use amethyst::input::VirtualKeyCode;
use std::collections::HashSet;

pub struct DownKeys {
    pub old_keys: HashSet<VirtualKeyCode>,
    pub new_keys: HashSet<VirtualKeyCode>,
}

impl DownKeys {
    pub fn update(&mut self, key_iter: impl Iterator<Item = VirtualKeyCode>) {
        self.old_keys = self.new_keys.clone();
        self.new_keys = HashSet::new();
        for key in key_iter {
            self.new_keys.insert(key);
        }
    }

    pub fn key_ups(&self) {
        self.old_keys.difference(&self.new_keys)
    }
}

impl Default for DownKeys {
    fn default() -> Self {
        DownKeys {
            old_keys: HashSet::new(),
            new_keys: HashSet::new(),
        }
    }
}
