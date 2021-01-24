use super::spawnable::Spawnable;
use super::components::Components;
use super::transform2::Transform2;

pub struct PointInSpace { }

impl Spawnable for PointInSpace {
    fn components_to_spawn_with() -> Components {
        Components::with_capacity(2).add_default::<Transform2>().unwrap()
    }
}