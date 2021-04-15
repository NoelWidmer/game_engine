mod ecs;
mod transform;
mod math;
mod camera;

use camera::CameraComponent;
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

    let pis1_t = world.component::<Transform2>(pis1_id).unwrap();
    let pis2_t = world.component::<Transform2>(pis2_id).unwrap();
    
    println!("{:?}", pis1_t);
    println!("{:?}", pis2_t);

    spawn_camera(&mut world);
}

fn spawn_camera(world: &mut World) {
    let entity_id = world.spawn_entity();

    let mut transform = Transform2::new_orphan();
    transform.set_location(Vec2::new(10f32, 20f32));
    world.add_component(entity_id, transform).unwrap();

    let mut camera = CameraComponent::new(Vec2::new(1920f32, 1080f32), 4.5f32);
    world.add_component(entity_id, camera).unwrap();
}
