use winit::keyboard::KeyCode;
use bevy_ecs::prelude::Resource;

use crate::input::{keyboard::KeyboardState, mouse::MouseState};

#[derive(Resource, Default, Debug)]
pub struct InputState {
    pub(crate) keyboard: KeyboardState,
    pub(crate) mouse: MouseState,
}

impl InputState {   
    pub fn is_key_pressed(&self, key: KeyCode) -> bool {   
        self.keyboard.pressed_keys.contains(&key)
    }

    pub fn is_key_just_pressed(&self, key: KeyCode) -> bool {
        self.keyboard.just_pressed_keys.contains(&key)
    }

    pub fn is_key_just_released(&self, key: KeyCode) -> bool {
        self.keyboard.just_released_keys.contains(&key)
    }

    pub fn is_button_pressed(&self, button: winit::event::MouseButton) -> bool {
        self.mouse.pressed_buttons.contains(&button)
    }

    pub fn is_button_just_pressed(&self, button: winit::event::MouseButton) -> bool {
        self.mouse.just_pressed_buttons.contains(&button)
    }

    pub fn is_button_just_released(&self, button: winit::event::MouseButton) -> bool {
        self.mouse.just_released_buttons.contains(&button)
    }

    pub fn get_scroll_delta(&self) -> glam::Vec2 {
        self.mouse.scroll_delta
    }

    pub fn get_mouse_position(&self) -> glam::Vec2 {
        self.mouse.position
    }

    pub fn get_mouse_delta(&self) -> glam::Vec2 {
        self.mouse.delta
    }

    pub(crate) fn clear_frame_states(&mut self) { // Clear the just pressed and just released states at the end of each frame (Only engine can call this)
        self.keyboard.just_pressed_keys.clear();
        self.keyboard.just_released_keys.clear();
        self.mouse.just_pressed_buttons.clear();
        self.mouse.just_released_buttons.clear();
        self.mouse.scroll_delta = glam::Vec2::ZERO;
        self.mouse.delta = glam::Vec2::ZERO;
    }
}