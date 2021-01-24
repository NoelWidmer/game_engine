use super::components::Components;

pub trait Spawnable {
    fn components_to_spawn_with() -> Components;
}