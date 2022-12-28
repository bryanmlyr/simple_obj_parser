use macroquad::camera::{Camera3D, set_camera};
use macroquad::color::{BLACK, WHITE};
use macroquad::input::{is_key_down, KeyCode};
use macroquad::math::{vec3, Vec3};
use macroquad::models::draw_line_3d;
use macroquad::window::clear_background;

pub fn loop_controller(x: &mut f32, theta: &mut f32) {
    clear_background(BLACK);

    set_camera(&Camera3D {
        position: vec3(*x, 5., 0.),
        up: vec3(0., 1., 0.),
        target: vec3(0., 0., 0.),
        ..Default::default()
    });

    if is_key_down(KeyCode::Down) {
        *x += 1.0;
    }
    if is_key_down(KeyCode::Up) {
        *x -= 1.0;
    }
    if is_key_down(KeyCode::Left) {
        *theta += 0.05;
    }
    if is_key_down(KeyCode::Right) {
        *theta -= 0.05;
    }

}

pub fn drawer(faces: &Vec<Vec<i32>>, vertices: &Vec<Vec3>) {
    for face in faces {
        match face.len() {
            4 => {
                let v1: &Vec3 = vertices.get(face[0] as usize - 1).unwrap();
                let v2: &Vec3 = vertices.get(face[1] as usize - 1).unwrap();
                let v3: &Vec3 = vertices.get(face[2] as usize - 1).unwrap();
                let v4: &Vec3 = vertices.get(face[3] as usize - 1).unwrap();

                draw_line_3d(*v1, *v2, WHITE);
                draw_line_3d(*v2, *v3, WHITE);
                draw_line_3d(*v3, *v4, WHITE);
                draw_line_3d(*v4, *v1, WHITE);
            }
            3 => {
                let v1: &Vec3 = vertices.get(face[0] as usize - 1).unwrap();
                let v2: &Vec3 = vertices.get(face[1] as usize - 1).unwrap();
                let v3: &Vec3 = vertices.get(face[2] as usize - 1).unwrap();

                draw_line_3d(*v1, *v2, WHITE);
                draw_line_3d(*v2, *v3, WHITE);
                draw_line_3d(*v3, *v1, WHITE);
            }
            _ => {}
        }
    }
}
