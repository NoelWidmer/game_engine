mod ecs;
mod transform;
mod math;

use ecs::World;
use transform::Transform2;
use transform::Transformer2;
use math::Vec2;

fn main() {
    let mut world = World::new();

    let mut t = Transform2::new_orphan();
    t.set_location(Vec2::new(5f32, 10f32));

    let pis1_id = world.spawn_entity();
    world.add_component(pis1_id, t).unwrap();

    let pis2_id = world.spawn_entity();
    world.add_default_component::<Transform2>(pis2_id).unwrap();

    Transformer2::adopt(&mut world, pis1_id, pis2_id).unwrap();

    let pis1_t = world.get_component::<Transform2>(pis1_id).unwrap();
    let pis2_t = world.get_component::<Transform2>(pis2_id).unwrap();
    
    println!("{:?}", pis1_t);
    println!("{:?}", pis2_t);
}
