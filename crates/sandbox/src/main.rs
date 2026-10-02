use std::sync::Arc;

use engine::app::app::App;
use engine::render::camera::{self, Camera};
use engine::window::window::WindowConfig;
use engine::render::{context::RenderContext, mesh::{Mesh, Vertex}};
use engine::render::material::Material;
use engine::render::layout::GpuLayout;
use engine::render::texture::Texture;
use engine::render::camera::{GpuCamera, CameraUniform};
use engine::input::input::InputState;

use bevy_ecs::prelude::*;
use winit::keyboard::KeyCode;

const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.0868241, 0.49240386, 0.0], uv: [0.4131759, 0.00759614], }, // A
    Vertex { position: [-0.49513406, 0.06958647, 0.0], uv: [0.0048659444, 0.43041354], }, // B
    Vertex { position: [-0.21918549, -0.44939706, 0.0], uv: [0.28081453, 0.949397], }, // C
    Vertex { position: [0.35966998, -0.3473291, 0.0], uv: [0.85967, 0.84732914], }, // D
    Vertex { position: [0.44147372, 0.2347359, 0.0], uv: [0.9414737, 0.2652641], }, // E
];

const INDICES: &[u16] = &[
    0, 1, 4,
    1, 2, 4,
    2, 3, 4,
];

fn main() {
    App::new()
        .set_window(WindowConfig {
            title: "Space Simulator".to_string(),
            width: 1280,
            height: 720,
        })
        .add_startup_system(test)
        .add_system(camera_system)
        .run();
}

fn test(mut commands: Commands, render_context: Res<RenderContext>, gpu_layout: Res<GpuLayout>) {
    let camera =  Camera {
        eye: glam::Vec3::new(0.0, 1.0, 2.0),
        target: glam::Vec3::new(0.0, 0.0, 0.0),
        up: glam::Vec3::new(0.0, 1.0, 0.0),
        aspect: render_context.config.width as f32 / render_context.config.height as f32,
        fovy: 45.0_f32.to_radians(),
        znear: 0.1,
        zfar: 100.0,
    };

    let mesh = Mesh::new(&render_context.device, VERTICES, INDICES);

    let texture = Texture::from_bytes(&render_context.device, &render_context.queue, include_bytes!("../../../assets/textures/happy-tree.png"), "happy-tree.png").unwrap();

    let material = Material::new(&render_context.device, &gpu_layout.material, Arc::new(texture));

    commands.spawn((mesh, material));
    commands.spawn(camera);
}

fn camera_system(mut query: Query<&mut Camera>, render_context: Res<RenderContext>, camera_gpu: Res<engine::render::camera::GpuCamera>, input: Res<InputState>) {
    let speed: f32 = 0.2;

    for mut camera in query.iter_mut() {
        let mut camera_uniform = CameraUniform::new();
        let forward = camera.target - camera.eye;
        let forward_norm = forward.normalize();
        let forward_mag = forward.length();

        if input.is_key_pressed(KeyCode::KeyW) && forward_mag > speed {
            camera.eye += forward_norm * speed;
        }
        else if input.is_key_pressed(KeyCode::KeyS) {
            camera.eye -= forward_norm * speed;
        }
        
        let right = forward_norm.cross(camera.up);

        let forward = camera.target - camera.eye;
        let forward_mag = forward.length();

        if input.is_key_pressed(KeyCode::KeyA) {
            camera.eye = camera.target - (forward + right * speed).normalize() * forward_mag;
        }
        else if input.is_key_pressed(KeyCode::KeyD) {
            camera.eye = camera.target - (forward - right * speed).normalize() * forward_mag;
        }

        camera_uniform.update_view_proj(&camera);
        render_context.queue.write_buffer(&camera_gpu.uniform_buffer, 0, bytemuck::cast_slice(&[camera_uniform]));
    }
}