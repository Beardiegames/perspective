use winit::{
    event::*,
    window::Window,
    dpi::PhysicalSize,
};
use crate::*;
use cgmath::prelude::*;




pub struct State {
    canvas: Canvas,
    render_pipeline: wgpu::RenderPipeline,
    
    vertex_buffer: wgpu::Buffer,
    num_vertices: u32,
    vertex_index_buffer: wgpu::Buffer,
    num_indices: u32,
    
    diffuse_bind_group: wgpu::BindGroup,
    diffuse_texture: texture::Texture,
    
    camera: Camera,
    camera_uniform: CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    
    instances: Vec<Instance>,
    instance_buffer: wgpu::Buffer,
    depth_texture: Texture,
    
    camera_controller: CameraController,
}

impl State {
    // Creating some of the wgpu types requires async code
    pub async fn new(window: Window) -> Self {        
        let canvas = Canvas::new(window).await;
            
        // load texture image
        let diffuse_bytes = include_bytes!("../assets/models/GrassTile.png");
        let diffuse_texture = texture::Texture::from_bytes(&canvas, diffuse_bytes, "../assets/models/GrassTile.png").unwrap(); // CHANGED!
        let (texture_bind_group_layout, diffuse_bind_group) = texture::create_diffuse_bindgroup(&canvas, &diffuse_texture);
                  
        // setup render camera
        let camera = Camera {
            eye: (0.0, 1.0, 2.0).into(), // position the camera one unit up and 2 units back, +z is out of the screen
            target: (0.0, 0.0, 0.0).into(), // have it look at the origin
            up: cgmath::Vector3::unit_y(), // which way is "up"
            aspect: canvas.aspect_ratio(),
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        };
        
        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(&camera);
        
        let camera_buffer = create_camera_buffer(&canvas, camera_uniform);
        let (camera_bind_group_layout, camera_bind_group) = create_camera_bindgroup(&canvas, &camera_buffer);
                  
        // setup vertex and indices buffers
        let (vertex_buffer, num_vertices) = vertices::new_vertex_buffer(&canvas.device);
        let (vertex_index_buffer, num_indices) = vertices::new_index_buffer(&canvas.device);
        
        // insantiating multiple render objects
        let instances = setup_instance_data();
        let instance_buffer = create_instance_buffer(&canvas, &instances);
                  
        // render depth buffering
        let depth_texture = texture::Texture::create_depth_texture(&canvas, "depth_texture");
        
                  
        // setup render pipeline
        let shader = shaders::create_shader(&canvas.device);
        
        let buffer_layouts = &[Vertex::desc(), InstanceRaw::desc()];
        let bindgroup_layouts = &[&texture_bind_group_layout, &camera_bind_group_layout];
        
        let render_pipeline = render_pipeline::create_new(
            &canvas.device,
            &shader,
            canvas.config.format,
            buffer_layouts,
            bindgroup_layouts,
        );

        // controller for camera movement -> This needs to be refactored into user implementation
        let camera_controller = CameraController::new(0.2);
        
        Self {
            canvas,
            render_pipeline,
            
            vertex_buffer,
            num_vertices,
            vertex_index_buffer,
            
            num_indices,
            diffuse_bind_group,
            diffuse_texture,
            
            camera,
            camera_uniform,
            camera_buffer,
            camera_bind_group,
            
            instances,
            instance_buffer,
            depth_texture,
            
            camera_controller,
        }
    }

    pub fn window(&self) -> &Window {
        &self.canvas.window
    }
    
    pub fn reconfigure_surface(&self) {
        self.canvas.reconfigure_surface();
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if self.canvas.resize(new_size) {
            self.depth_texture = Texture::create_depth_texture(&self.canvas, "depth_texture");  
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        self.camera_controller.process_events(event)
    }

    pub fn update(&mut self) {
        self.camera_controller.update_camera(&mut self.camera);
        self.camera_uniform.update_view_proj(&self.camera);
        self.canvas.queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[self.camera_uniform]));
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.canvas.surface.get_current_texture()?;
        let canvas = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        let mut encoder = self.canvas.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &canvas,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });
            
            render_pass.set_pipeline(&self.render_pipeline);
            
            render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
            render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
            
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            render_pass.set_index_buffer(self.vertex_index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            
            render_pass.draw_indexed(0..self.num_indices, 0, 0..self.instances.len() as _);
        }
    
        // submit will accept anything that implements IntoIter
        self.canvas.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    
        Ok(())
    }
}

struct CameraController {
    speed: f32,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
}

impl CameraController {
    fn new(speed: f32) -> Self {
        Self {
            speed,
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
        use cgmath::InnerSpace;
        let forward = camera.target - camera.eye;
        let forward_norm = forward.normalize();
        let forward_mag = forward.magnitude();

        // Prevents glitching when camera gets too close to the
        // center of the scene.
        if self.is_forward_pressed && forward_mag > self.speed {
            camera.eye += forward_norm * self.speed;
        }
        if self.is_backward_pressed {
            camera.eye -= forward_norm * self.speed;
        }

        let right = forward_norm.cross(camera.up);

        // Redo radius calc in case the fowrard/backward is pressed.
        let forward = camera.target - camera.eye;
        let forward_mag = forward.magnitude();

        if self.is_right_pressed {
            // Rescale the distance between the target and eye so 
            // that it doesn't change. The eye therefore still 
            // lies on the circle made by the target and eye.
            camera.eye = camera.target - (forward + right * self.speed).normalize() * forward_mag;
        }
        if self.is_left_pressed {
            camera.eye = camera.target - (forward - right * self.speed).normalize() * forward_mag;
        }
    }
}
