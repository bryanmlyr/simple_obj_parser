use macroquad::camera::set_default_camera;
use macroquad::time::get_frame_time;
use macroquad::window::next_frame;

use crate::obj::{apply_rotation, extract_faces, extract_vertices};
use crate::engine::{drawer, loop_controller};

mod obj;
mod engine;

#[macroquad::main("obj reader")]
async fn main() {
    let path = "./examples/head.obj";

    let faces = extract_faces(path);
    let vertices= extract_vertices(path, 1.);

    let mut theta = 0.;
    let mut x = -25.;

    let minimum_frame_time = 1. / 25.; // 25 FPS

    loop {
        loop_controller(&mut x, &mut theta);

        drawer(&faces,&apply_rotation(&vertices, theta));

        let frame_time = get_frame_time();
        if frame_time < minimum_frame_time {
            let time_to_sleep = (minimum_frame_time - frame_time) * 1000.;

            std::thread::sleep(std::time::Duration::from_millis(time_to_sleep as u64));
        }

        theta += 0.005;

        set_default_camera();
        next_frame().await
    }
}
