use gltf::Gltf;
use gltf_animation::Animation;

fn main() {
    let gltf = Gltf::open("assets/example-scene.glb").expect("Could not create glTF structure");
    let mut animation = Animation::from_gltf(&gltf, "CubeAction")
        .expect("Animation doesn't exist for given target");

    let frames_per_second = 60.0;
    let delta_seconds = 1.0 / frames_per_second;

    let simulation_duration = 4.0;

    // Simulate 4 seconds
    for _ in 0..((frames_per_second * simulation_duration) as i32) {
        animation.update(delta_seconds);
        println!("{:?}", animation.timeline_position());
        println!("XYZ: {:?}", animation.current_translation("Cube").unwrap());
        println!("Quat: {:?}\n", animation.current_rotation("Cube").unwrap());
    }
}
