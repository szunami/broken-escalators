pub const FPS_SYSTEM: &str = "fps_system";
pub const THING_TAPE_SYSTEM: &str = "thing_tape_system";
pub const STEP_TAPE_SYSTEM: &str = "step_tape_system";
pub const DOWN_KEY_SYSTEM: &str = "down_key_system";
pub const TOGGLE_SYSTEM: &str = "toggle_system";
pub const PLATFORM_SYSTEM: &str = "platform_system";
pub const REWINDABLE_CLOCK_SYSTEM: &str = "rewindable_clock_system";

pub fn core_systems() -> Vec<&'static str> {
    vec![FPS_SYSTEM, THING_TAPE_SYSTEM, STEP_TAPE_SYSTEM, DOWN_KEY_SYSTEM, TOGGLE_SYSTEM, PLATFORM_SYSTEM, REWINDABLE_CLOCK_SYSTEM]
}

pub const CORNER_SYSTEM: &str = "corner_system";
pub const ATOP_SYSTEM: &str = "atop_system";

pub fn atop_dependencies() -> Vec<&'static str> {
    let x = core_systems();
    core_systems().push(ATOP_SYSTEM);
    x
}

pub fn velocity_systems() -> Vec<&'static str> {
    vec![CORNER_SYSTEM, ATOP_SYSTEM]
}

pub const ESCALATOR_SYSTEM: &str = "escalator_system";
pub const MOVE_SYSTEM: &str = "move_system";
