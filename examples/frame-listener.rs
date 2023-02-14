use gltf::Gltf;
use gltf_animation::Animation;

fn main() {
    let gltf = Gltf::open("assets/example-scene.glb").expect("Could not create glTF structure");
    let anim_name = "CubeAction";
    let mut animation =
        Animation::from_gltf(&gltf, anim_name).expect("Animation doesn't exist for given target");

    let frames_per_second = animation.fps() as f32;
    let delta_seconds = 1.0 / frames_per_second;
    let simulation_duration = 0.25;
    let item_name = "Cube";

    animation.on_frame(5, || {
        println!("#####");
        println!("## Frame 5 for {} was reached\n", anim_name);
        println!("#####\n");
    });

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
