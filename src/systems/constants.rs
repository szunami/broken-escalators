use super::{
    core::{
        DownKeysSystem, FPSSystem, PlatformSystem, RewindableClockSystem, StepTapeSystem,
        ThingTapeSystem, ToggleSystem,
    },
    position::MoveSystem,
    velocity::{AtopSystem, CornerSystem},
};
use std::any;

pub fn core_systems() -> Vec<&'static str> {
    vec![
        any::type_name::<FPSSystem>(),
        any::type_name::<ThingTapeSystem>(),
        any::type_name::<StepTapeSystem>(),
        any::type_name::<DownKeysSystem>(),
        any::type_name::<ToggleSystem>(),
        any::type_name::<PlatformSystem>(),
        any::type_name::<RewindableClockSystem>(),
    ]
}

pub fn atop_dependencies() -> Vec<&'static str> {
    let x = core_systems();
    core_systems().push(any::type_name::<AtopSystem>());
    x
}

pub fn velocity_systems() -> Vec<&'static str> {
    vec![
        any::type_name::<CornerSystem>(),
        any::type_name::<AtopSystem>(),
    ]
}

pub fn position_systems() -> Vec<&'static str> {
    vec![any::type_name::<MoveSystem>()]
}
