use gltf::Gltf;
use gltf_animation::Animation;

fn main() {
    let gltf = Gltf::open("assets/skeleton.glb").expect("Could not create glTF structure");
    let mut animation = Animation::from_gltf(&gltf, "ArmatureAction.001")
        .expect("Animation doesn't exist for given target");

    let frames_per_second = 60.0;
    let delta_seconds = 1.0 / frames_per_second;

    let simulation_duration = 1.0;

    let item_name = "bone_2";

    // Simulate 4 seconds
    for _ in 0..((frames_per_second * simulation_duration) as i32) {
        animation.update(delta_seconds);

        println!("{}", item_name);
        println!("{:?}", animation.timeline_position());
        println!(
            "XYZ: {:?}",
            animation.current_translation(item_name).unwrap()
        );
        println!("Quat: {:?}", animation.current_rotation(item_name).unwrap());
        println!("");
    }
}
