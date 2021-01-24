mod vec2;

mod transform2;
mod point_in_space;

mod world;
mod spawnable;
mod entity;
mod components;

use world::World;
use transform2::Transform2;
use point_in_space::PointInSpace;

fn main() {
    let mut world = World::new();

    let pis1 = world.spawn_entity::<PointInSpace>();
    println!("added pis: {}", pis1);

    let pis2 = world.spawn_entity::<PointInSpace>();
    println!("added pis: {}", pis2);

    let pis = world.find_entities_with_component::<Transform2>();
    println!("number of pis: {}", pis.len());

    world.despawn_entity(&pis1);
    println!("despawned pis: {}", pis1);

    world.despawn_entity(&pis2);
    println!("despawned pis: {}", pis2);
}
