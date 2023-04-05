//use perspective::run;
use perspective::*;

fn main() {
    perspective::run(Game  {
        cc: CameraController::new(0.1),
    });
}

struct Game {
    cc: CameraController,
}

impl GfxHandler for Game {
    fn on_startup(&mut self, gfx: &mut Gfx) {
        gfx.camera.projection = Projection::Otrho { size: 10.0 };
    }
    
    fn on_input(&mut self, event: &WindowEvent) -> bool {
        self.cc.process_events(event)
    }

    fn on_update(&mut self, gfx: &mut Gfx) {
        self.cc.update_camera(&mut gfx.camera);
    }
    
    fn on_resize(&mut self, _gfx: &mut Gfx) {
        
    }
}

trait ToFloat {
    fn to_float(&self) -> f32;
}

impl ToFloat for bool {
    fn to_float(&self) -> f32 {
        match self {
            true => 1.0,
            false => 0.0,
        }
    }
}

struct CameraController {
    speed: f32,
    is_ortho: bool,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
}

impl CameraController {
    fn new(speed: f32) -> Self {
        Self {
            speed,
            is_ortho: false,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
        }
    }

    fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input: KeyboardInput {
                    state,
                    virtual_keycode: Some(keycode),
                    ..
                },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                match keycode {
                    VirtualKeyCode::Space => {
                        self.is_ortho = !self.is_ortho;
                        true
                    }
                    VirtualKeyCode::W | VirtualKeyCode::Up => {
                        self.is_forward_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::A | VirtualKeyCode::Left => {
                        self.is_left_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::S | VirtualKeyCode::Down => {
                        self.is_backward_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::D | VirtualKeyCode::Right => {
                        self.is_right_pressed = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }
    
    
    fn update_camera(&self, camera: &mut Camera) {
        
        let fy = self.is_forward_pressed.to_float() * -1.0 + self.is_backward_pressed.to_float();
        let fx = self.is_right_pressed.to_float() + self.is_left_pressed.to_float() * -1.0;
        
        camera.eye.x += fx * self.speed;
        camera.target.x += fx * self.speed;
        
        camera.eye.z += fy * self.speed;
        camera.target.z += fy * self.speed;
        
        // camera.projection = match self.is_ortho {
        //     true => Projection::Otrho { size: 3.0 },
        //     false => Projection::Perspective { fov: 45.0 },
        // }
        
        
        // use cgmath::InnerSpace;
        // let forward = camera.target - camera.eye;
        // let forward_norm = forward.normalize();
        // let forward_mag = forward.magnitude();

        // // Prevents glitching when camera gets too close to the
        // // center of the scene.
        // if self.is_forward_pressed && forward_mag > self.speed {
        //     camera.eye += forward_norm * self.speed;
        // }
        // if self.is_backward_pressed {
        //     camera.eye -= forward_norm * self.speed;
        // }

        // let right = forward_norm.cross(camera.up);

        // // Redo radius calc in case the fowrard/backward is pressed.
        // let forward = camera.target - camera.eye;
        // let forward_mag = forward.magnitude();

        // if self.is_right_pressed {
        //     // Rescale the distance between the target and eye so 
        //     // that it doesn't change. The eye therefore still 
        //     // lies on the circle made by the target and eye.
        //     camera.eye = camera.target - (forward + right * self.speed).normalize() * forward_mag;
        // }
        // if self.is_left_pressed {
        //     camera.eye = camera.target - (forward - right * self.speed).normalize() * forward_mag;
        // }
    }
}
