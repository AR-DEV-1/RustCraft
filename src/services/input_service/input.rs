use crate::services::settings_service::key_mappings::KeyMapping;
use nalgebra::Vector2;
use std::borrow::Borrow;
use std::sync::Arc;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{ElementState, MouseButton, VirtualKeyCode, WindowEvent};
use winit::window::Window;

#[derive(PartialEq)]
pub enum InputChange {
    Pressed,
    Released,
    None,
}

/// Tracks user input_service's since the last frame.
/// Naming them things like movement instead of WASD keys makes it easier to support multiple input_service device types.
pub struct InputState {
    pub movement: [i32; 2],
    pub look: [f64; 2],
    pub use_item: bool,
    pub activate_item: bool,
    pub pause: bool,
    pub debugging: bool,
    pub jump: bool,
    pub sneak: bool,
    pub ctrl: InputChange,
    pub mouse: Vector2<f32>,

    pub mappings: KeyMapping,
    pub mouse_home: PhysicalPosition<u32>,

    /// Is the window hiding the cursor
    pub grabbed: bool,

    /// Should the window have hide the cursor
    pub attempt_grab: bool,

    window: Arc<Window>,
}

impl InputState {
    pub fn new(window: Arc<Window>) -> InputState {
        InputState {
            movement: [0; 2],
            look: [0.0; 2],
            use_item: false,
            activate_item: false,
            pause: false,
            debugging: false,
            jump: false,
            sneak: false,
            ctrl: InputChange::None,
            mouse: Vector2::zeros(),
            mappings: KeyMapping::default(),
            mouse_home: PhysicalPosition::new(0, 0),
            grabbed: false,
            attempt_grab: false,
            window,
        }
    }

    pub fn clear_ui(&mut self) {
        self.pause = false;
        self.debugging = false;
    }

    pub fn clear_physics(&mut self) {
        self.look = [0.0; 2];
    }

    fn item_used(&mut self) {
        self.use_item = true;
    }

    fn item_activated(&mut self) {
        self.activate_item = true;
    }

    fn cursor_position(&mut self, new: Vector2<f32>) {
        self.mouse = new;
    }

    pub fn resized(&mut self, size: &PhysicalSize<u32>) {
        self.mouse_home = PhysicalPosition {
            x: size.width / 2,
            y: size.height / 2,
        };
    }

    //TODO: Eventually move this into a separate class so its easier to hook in controller game_changes

    /// Converts keyboard input_service game_changes into the different actions they perform.
    pub fn handle_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::MouseInput {
                device_id: _,
                state: _,
                button,
                ..
            } => {
                if button == &MouseButton::Left {
                    self.item_used();
                } else if button == &MouseButton::Right {
                    self.item_activated();
                }
            }

            WindowEvent::KeyboardInput {
                device_id: _device_id,
                input,
                is_synthetic: _,
            } => {
                if input.virtual_keycode != None {
                    let key = input.virtual_keycode.unwrap();

                    self.handle_keyboard_input(input.state == ElementState::Pressed, key);
                }
            }

            WindowEvent::CursorMoved {
                device_id: _device_id,
                position,
                ..
            } => {
                self.cursor_position(Vector2::new(position.x as f32, position.y as f32));

                if self.grabbed {
                    let raw_x = position.x as f64;
                    let raw_y = position.y as f64;

                    let x = -1.0 * (raw_x - self.mouse_home.x as f64);
                    let y = -1.0 * (raw_y - self.mouse_home.y as f64);

                    self.look[0] += x;
                    self.look[1] += y;

                    if let Err(e) =
                        (self.window.borrow() as &Window).set_cursor_position(self.mouse_home)
                    {
                        log_error!("Error setting cursor position: {}", e);
                    }
                }
            }
            _ => {}
        }
    }

    fn handle_keyboard_input(&mut self, pressed: bool, key: VirtualKeyCode) {
        if key == self.mappings.pause {
            self.pause = pressed;
        }
        if key == self.mappings.debugging {
            self.debugging = pressed;
        }

        // Everything here on is game controls, so ignore if not grabbed
        if !self.grabbed {
            return;
        }

        if key == self.mappings.jump {
            self.jump = pressed;
        }

        if key == self.mappings.sneak {
            self.sneak = pressed;
        }

        if pressed {
            if key == self.mappings.forwards {
                self.movement[0] = 1;
            }

            if key == self.mappings.backwards {
                self.movement[0] = -1;
            }

            if key == self.mappings.left {
                self.movement[1] = 1;
            }

            if key == self.mappings.right {
                self.movement[1] = -1;
            }

            if key == self.mappings.ctrl {
                self.ctrl = InputChange::Pressed;
            }
        } else {
            if key == self.mappings.forwards || key == self.mappings.backwards {
                self.movement[0] = 0;
            }

            if key == self.mappings.left || key == self.mappings.right {
                self.movement[1] = 0;
            }

            if key == self.mappings.ctrl {
                self.ctrl = InputChange::Released;
            }
        }
    }

    /// Sets the state of the application that the mouse should be captured
    pub fn set_capture_mouse(&mut self) {
        self.capture_mouse();
        self.attempt_grab = true;
    }

    pub fn capture_mouse(&mut self) {
        if let Err(e) = self.window.set_cursor_grab(true) {
            log_error!("Error grabbing cursor: {}", e);
        }
        self.window.set_cursor_visible(false);
        if let Err(e) = self.window.set_cursor_position(self.mouse_home) {
            log_error!("Error setting cursor position: {}", e);
        }
        self.grabbed = true;
    }

    /// Sets the state of the application that the mouse should be captured
    pub fn set_uncapture_mouse(&mut self) {
        self.uncapture_mouse();
        self.attempt_grab = false;
    }

    pub fn uncapture_mouse(&mut self) {
        if let Err(e) = self.window.set_cursor_grab(false) {
            log_error!("Error releasing cursor: {}", e);
        }
        self.window.set_cursor_visible(true);
        self.grabbed = false;
    }
}

impl<'a> Default for InputState {
    fn default() -> Self {
        unimplemented!()
    }
}
