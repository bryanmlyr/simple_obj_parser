use std::fs;
use macroquad::math::{vec3, Vec3};

pub fn rotate(vec: &Vec3, theta: f32) -> Vec3 {
    // x' = z * sin(θ) + x * cos(θ)
    // y' = y
    // z' = z * cos(θ) - x * sin(θ)
    vec3(vec.z * theta.sin() + vec.x * theta.cos(), vec.y, vec.z * theta.cos() - vec.x * theta.sin())
}

pub fn extract_vertices(path: &str, zoom: f32) -> Vec<Vec3> {
    let file_content = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    file_content
        .lines()
        .filter(|x| x.starts_with("v "))
        .map(|str| {
            let clean = str.replace("v ", "");
            let splits: Vec<&str> = clean.split(" ").collect::<Vec<&str>>();

            let x = splits[0].parse::<f32>().unwrap() * zoom;
            let y = splits[1].parse::<f32>().unwrap() * zoom;
            let z = splits[2].parse::<f32>().unwrap() * zoom;

            vec3(x, y, z)
        })
        .collect::<Vec<Vec3>>()
}

pub fn extract_faces(path: &str) -> Vec<Vec<i32>> {
    let file_content = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    file_content
        .lines()
        .filter(|x| x.starts_with("f "))
        .map(|str| str.replace("f ", "").split(" ").map(|e| {
            let face_v = e.trim().split("/").filter(|r| !r.is_empty()).collect::<Vec<&str>>();

            face_v[0].parse::<i32>().unwrap()
        }).collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>()
}

pub fn apply_rotation(vertices: &Vec<Vec3>, theta: f32) -> Vec<Vec3> {
    vertices.iter().map(|x| rotate(&x, theta)).collect::<Vec<Vec3>>()
}
