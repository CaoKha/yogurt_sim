use winit::{event::*, window::Window};
mod buffer;
use buffer::Buffer;
mod surface;

mod render_pipeline;

mod vertex;
use vertex::Vertex;

mod texture;

#[rustfmt::skip]
const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.0868241, 0.49240386, 0.0], tex_coords: [0.4131759, 0.00759614], }, // A
    Vertex { position: [-0.49513406, 0.06958647, 0.0], tex_coords: [0.0048659444, 0.43041354], }, // B
    Vertex { position: [-0.21918549, -0.44939706, 0.0], tex_coords: [0.28081453, 0.949397], }, // C
    Vertex { position: [0.35966998, -0.3473291, 0.0], tex_coords: [0.85967, 0.84732914], }, // D
    Vertex { position: [0.44147372, 0.2347359, 0.0], tex_coords: [0.9414737, 0.2652641], }, // E
];

const VERTICES_DIF_COLOR: &[Vertex] = &[
    Vertex {
        position: [-0.0868241, 0.49240386, 0.0],
        tex_coords: [0.00759614, 0.4131759],
    }, // A
    Vertex {
        position: [-0.49513406, 0.06958647, 0.0],
        tex_coords: [0.43041354, 0.0048659444],
    }, // B
    Vertex {
        position: [-0.21918549, -0.44939706, 0.0],
        tex_coords: [0.949397, 0.28081453],
    }, // C
    Vertex {
        position: [0.35966998, -0.3473291, 0.0],
        tex_coords: [0.84732914, 0.85967],
    }, // D
    Vertex {
        position: [0.44147372, 0.2347359, 0.0],
        tex_coords: [0.2652641, 0.9414737],
    }, // E
];
// const VERTICES: &[Vertex] = &[
//     Vertex { position: [0.0, 0.5, 0.0], color: [1.0, 0.0, 0.0] },
//     Vertex { position: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] },
//     Vertex { position: [0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0] },
// ];

// const VERTICES_DIF_COLOR: &[Vertex] = &[
//     Vertex {
//         position: [0.0, 0.5, 0.0],
//         color: [0.5, 0.0, 0.5],
//     },
//     Vertex {
//         position: [-0.5, -0.5, 0.0],
//         color: [0.5, 0.0, 0.5],
//     },
//     Vertex {
//         position: [0.5, -0.5, 0.0],
//         color: [0.5, 0.0, 0.5],
//     },
// ];

#[rustfmt::skip]
const INDICES: &[u16] = &[
    0, 1, 4,
    1, 2, 4,
    2, 3, 4,
];

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
    num_vertices: u32,
    use_color: bool,
    diffuse_bind_group: wgpu::BindGroup, // NEW
}

impl State {
    // Creating some of the wgpu types requires async code
    pub async fn new(window: Window) -> Self {
        let size = window.inner_size();
        let num_vertices = VERTICES.len() as u32;
        let use_color = true;

        let (surface, adapter) = Self::init_surface(&window).await;
        let (device, queue) = Self::init_device(&adapter).await;
        let config = Self::init_config(size, &surface, &adapter);
        surface.configure(&device, &config);

        // NEW
        let texture_bind_group_layout = device.create_bind_group_layout(
            &texture::Texture::bind_group_layout_descriptor("texture_bind_group_layout"),
        );

        let diffuse_bytes = include_bytes!("happy-tree.png"); // CHANGED!
        let diffuse_texture =
            texture::Texture::from_bytes(&device, &queue, diffuse_bytes, "happy-tree.png").unwrap(); // CHANGED!

        let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                },
            ],
            label: Some("diffuse_bind_group"),
        });

        // END NEW

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
        let render_pipeline = Self::init_render_pipeline(
            &device,
            &config,
            &shader,
            "vs_main",
            "fs_main",
            &[&texture_bind_group_layout],
        );

        let clear_color = wgpu::Color::BLACK;

        let vertex_buffer = Buffer::new(
            &device,
            VERTICES,
            wgpu::BufferUsages::VERTEX,
            "Vertex Buffer",
        );

        let index_buffer = Some(Buffer::new(
            &device,
            INDICES,
            wgpu::BufferUsages::INDEX,
            "Index Buffer",
        ));

        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            clear_color,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_vertices,
            use_color,
            diffuse_bind_group,
            // diffuse_texture,
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
                self.use_color = *state == ElementState::Released;
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
        match self.use_color {
            true => {
                self.vertex_buffer = Buffer::new(
                    &self.device,
                    VERTICES,
                    wgpu::BufferUsages::VERTEX,
                    "Vertex Buffer",
                )
            }
            false => {
                self.vertex_buffer = Buffer::new(
                    &self.device,
                    VERTICES_DIF_COLOR,
                    wgpu::BufferUsages::VERTEX,
                    "Challenge Vertex Buffer",
                )
            }
        };

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

        render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);

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
        render_pass.draw(0..self.num_vertices, 0..1);
    }
}
