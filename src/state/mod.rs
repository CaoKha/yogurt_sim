use winit::{event::*, window::Window};

mod init_utils;
use init_utils::{init_config, init_device, init_surface, init_vertex_buffer};

mod render_pipeline;
use render_pipeline::*;

mod vertex;
use vertex::Vertex;

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.0, 0.5, 0.0],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [-0.5, -0.5, 0.0],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [0.5, -0.5, 0.0],
        color: [0.0, 0.0, 1.0],
    },
];

pub struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,

    clear_color: wgpu::Color, // Challenge tutorial Surface

    render_pipeline: RenderPipeline,

    vertex_buffer: wgpu::Buffer,
    num_vertices: u32,
}

impl State {
    // Creating some of the wgpu types requires async code
    pub async fn new(window: &Window) -> Self {
        let (surface, adapter) = init_surface(window).await;

        let size = window.inner_size();
        let config = init_config(size, &surface, &adapter);

        let (device, queue) = init_device(&adapter).await;
        surface.configure(&device, &config);

        let render_pipeline = RenderPipeline::new(&device, &config);

        let clear_color = wgpu::Color::BLACK;

        let vertex_buffer = init_vertex_buffer(&device, VERTICES);
        let num_vertices = VERTICES.len() as u32;

        Self {
            surface,
            device,
            queue,
            config,
            size,
            clear_color,
            render_pipeline,
            vertex_buffer,
            num_vertices,
        }
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
        match event {
            &WindowEvent::KeyboardInput {
                // Challenge tutorial Pipeline
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Space),
                        ..
                    },
                ..
            } => {
                self.render_pipeline.swap_pipeline();
                true
            }
            &WindowEvent::CursorMoved {
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
                true
            }
            _ => false,
        }
    }

    pub fn update(&mut self) {
        // todo!()
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

        self.render_pipeline.add_render_pass(
            &mut encoder,
            &view,
            self.clear_color,
            &self.vertex_buffer,
            self.num_vertices,
        );

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
