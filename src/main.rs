mod vec2;

mod transform2;
mod transformer2;

mod world;
mod entity;
mod components;

use world::World;
use components::Components;
use transform2::Transform2;

fn main() {
    let mut world = World::new();
    
    let pis = Components
        ::with_capacity(1)
        .add_default::<Transform2>().unwrap();

    let pis1 = world.spawn_entity(pis);
    println!("added pis: {}", pis1);

    let pis = Components
        ::with_capacity(1)
        .add_default::<Transform2>().unwrap();

    let pis2 = world.spawn_entity(pis);
    println!("added pis: {}", pis2);

    let pis = world.find_entities_with_component::<Transform2>();
    println!("number of pis: {}", pis.len());

    world.despawn_entity(&pis1);
    println!("despawned pis: {}", pis1);

    world.despawn_entity(&pis2);
    println!("despawned pis: {}", pis2);
}
