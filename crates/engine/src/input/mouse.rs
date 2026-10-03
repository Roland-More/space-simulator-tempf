use std::collections::HashSet;

use glam::Vec2;

use winit::event::MouseButton;

#[derive(Debug, Default)]
pub struct MouseState {
    pub position: Vec2,
    pub delta: Vec2,
    pub scroll_delta: Vec2,
    pub pressed_buttons: HashSet<MouseButton>,
    pub just_pressed_buttons: HashSet<MouseButton>,
    pub just_released_buttons: HashSet<MouseButton>,
}