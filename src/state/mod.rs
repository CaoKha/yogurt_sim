use wgpu::util::DeviceExt;
use winit::{event::*, window::Window};
mod builder;
mod components;
use components::{
    buffer::Buffer,
    camera::{Camera, CameraController, CameraUniform},
    texture::Texture,
    vertex::Vertex,
};

pub struct State {
    window: Window,
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    clear_color: wgpu::Color, // Challenge tutorial Surface
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: Buffer,
    index_buffer: Option<Buffer>,
    camera_buffer: wgpu::Buffer,
    // num_vertices: u32,
    num_indices: u32,
    bool_event: bool,
    default_texture: Texture,
    cartoon_texture: Texture,
    texture_bind_group: wgpu::BindGroup, // NEW
    camera_bind_group: wgpu::BindGroup,
    texture_bind_group_layout: wgpu::BindGroupLayout,
    camera: Camera,
    camera_controller: CameraController,
    camera_uniform: CameraUniform,
}

impl State {
    // Creating some of the wgpu types requires async code
    pub async fn new(window: Window) -> Self {
        // Window's config
        let window_size = window.inner_size();
        let (surface, adapter) = Self::init_surface(&window).await;
        let config = Self::init_surface_config(window_size, &surface, &adapter);
        let (device, queue) = Self::init_device(&adapter).await;
        surface.configure(&device, &config);

        // State's properties init
        let bool_event = true;
        let num_indices = Vertex::INDICES.len() as u32;

        // Textures
        let diffuse_bytes = include_bytes!("../../assets/happy-tree.png");
        let diffuse_texture =
            Texture::get_texture_from_bytes(&device, &queue, diffuse_bytes, "happy-tree-diffuse");
        let cartoon_bytes = include_bytes!("../../assets/happy-tree-cartoon.png");
        let cartoon_texture =
            Texture::get_texture_from_bytes(&device, &queue, cartoon_bytes, "happy-tree-cartoon");
        // Texture's bindgroup layout
        let texture_bind_group_layout =
            device.create_bind_group_layout(&Texture::get_bind_group_layout_descriptor());
        // Texture's bindgroup
        let diffuse_bind_group = Texture::init_texture_bind_group(
            &device,
            &diffuse_texture,
            &texture_bind_group_layout,
            "diffuse_texture_bind_group",
        );

        // Shaders
        let shader = device.create_shader_module(wgpu::include_wgsl!("../shaders/shader.wgsl"));

        // Buffers
        let vertex_buffer = Buffer::new(
            &device,
            Vertex::VERTICES_PENTAGON,
            wgpu::BufferUsages::VERTEX,
            "Vertex Buffer",
        );
        let index_buffer = Some(Buffer::new(
            &device,
            Vertex::INDICES,
            wgpu::BufferUsages::INDEX,
            "Index Buffer",
        ));

        // Camera
        let camera = Camera::new(&config);
        let camera_controller = CameraController::new(0.2);
        // Camera's Matrix
        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(&camera);
        // Camera's buffer
        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        // Camera's bindgroup layout
        let camera_bind_group_layout =
            device.create_bind_group_layout(&Camera::get_camera_bindgroup_layout_descriptor());
        // Camera's bindgroup
        let camera_bind_group = Camera::init_camera_bind_group(
            &device,
            &camera_bind_group_layout,
            "camera_bind_group",
            &camera_buffer,
        );

        // Render Pipeline
        let render_pipeline = Self::init_render_pipeline(
            &device,
            &config,
            &shader,
            "vs_main",
            "fs_main",
            &[&texture_bind_group_layout, &camera_bind_group_layout],
        );


        Self {
            window,
            surface,
            device,
            queue,
            config,
            size: window_size,
            clear_color: wgpu::Color::BLACK,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            bool_event,
            default_texture: diffuse_texture,
            cartoon_texture,
            texture_bind_group: diffuse_bind_group,
            camera_bind_group,
            texture_bind_group_layout,
            camera,
            camera_controller,
            camera_uniform,
            camera_buffer,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config)
        }
    }

    pub fn reset_size(&mut self) {
        self.resize(self.size)
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        self.camera_controller.process_events(event);
        match event {
            WindowEvent::KeyboardInput {
                // Challenge tutorial Pipeline
                input:
                    KeyboardInput {
                        state,
                        virtual_keycode: Some(VirtualKeyCode::Space),
                        ..
                    },
                ..
            } => {
                self.bool_event = *state == ElementState::Released;
                true
            }
            WindowEvent::CursorMoved {
                // Challenge tutorial Surface
                position: winit::dpi::PhysicalPosition { x, y },
                ..
            } => {
                self.clear_color = wgpu::Color {
                    r: x / self.size.width as f64,
                    g: y / self.size.height as f64,
                    b: x * y / (self.size.width as f64 * self.size.height as f64),
                    a: 1.0,
                };
                // self.clear_color = wgpu::Color {
                //     r: 0.1,
                //     g: 0.2,
                //     b: 0.3,
                //     a: 1.0,
                // };
                true
            }
            _ => false,
        }
    }

    pub fn update(&mut self) {
        // update camera
        self.camera_controller.update_camera(&mut self.camera);
        self.camera_uniform.update_view_proj(&self.camera);
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );

        // update texture via `bool_event`
        let angle = std::f32::consts::PI;
        let new_vertex = Vertex::rotate_vertex(angle, Vertex::VERTICES_PENTAGON);
        let new_indices = Vertex::INDICES;
        match self.bool_event {
            true => {
                self.vertex_buffer = Buffer::new(
                    &self.device,
                    Vertex::VERTICES_PENTAGON,
                    wgpu::BufferUsages::VERTEX,
                    "Vertex Buffer",
                );
                self.index_buffer = Some(Buffer::new(
                    &self.device,
                    Vertex::INDICES,
                    wgpu::BufferUsages::INDEX,
                    "Index Buffer",
                ));
                self.num_indices = Vertex::INDICES.len() as u32;
                self.texture_bind_group = Texture::init_texture_bind_group(
                    &self.device,
                    &self.default_texture,
                    &self.texture_bind_group_layout,
                    "diffuse_texture_bind_group",
                );
            }
            false => {
                self.vertex_buffer = Buffer::new(
                    &self.device,
                    &new_vertex,
                    wgpu::BufferUsages::VERTEX,
                    "Challenge Vertex Buffer",
                );
                self.index_buffer = Some(Buffer::new(
                    &self.device,
                    &new_indices,
                    wgpu::BufferUsages::INDEX,
                    "Index Buffer",
                ));
                self.num_indices = new_indices.len() as u32;
                self.texture_bind_group = Texture::init_texture_bind_group(
                    &self.device,
                    &self.cartoon_texture,
                    &self.texture_bind_group_layout,
                    "cartoon_texture_bind_group",
                );
            }
        };
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        self.add_render_pass(&mut encoder, &view, &self.render_pipeline);

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub fn add_render_pass(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        render_pipeline: &wgpu::RenderPipeline,
    ) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(self.clear_color),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        // update render pass
        render_pass.set_pipeline(render_pipeline);
        render_pass.set_bind_group(0, &self.texture_bind_group, &[]);
        render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
        self.vertex_buffer.attach_to(&mut render_pass);
        if let Some(index_buffer) = &self.index_buffer {
            index_buffer.attach_to(&mut render_pass);
        }
        if let Some(index_buffer) = &self.index_buffer {
            index_buffer.drawn_on(&mut render_pass);
        } else {
            self.vertex_buffer.drawn_on(&mut render_pass);
        }

        render_pass.set_vertex_buffer(0, (self.vertex_buffer.buffer).slice(..));
        if let Some(id_buffer) = &self.index_buffer {
            render_pass.set_index_buffer(id_buffer.buffer.slice(..), wgpu::IndexFormat::Uint16);
        }
        // render_pass.draw(0..self.num_vertices, 0..1);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1)
    }
}
