use winit::{event::*, window::Window};
mod buffer;
use buffer::Buffer;
mod render_pipeline;
mod surface;
mod vertex;
use vertex::Vertex;
mod texture;
use texture::Texture;

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
    num_indices: u32,
    use_color: bool,
    default_texture: Texture,
    bind_group: wgpu::BindGroup, // NEW
    texture_bind_group_layout: wgpu::BindGroupLayout,
}

impl State {
    // Creating some of the wgpu types requires async code
    pub async fn new(window: Window) -> Self {
        let size = window.inner_size();
        let num_vertices = Vertex::VERTICES_PENTAGON.len() as u32;
        let use_color = true;

        let (surface, adapter) = Self::init_surface(&window).await;
        let (device, queue) = Self::init_device(&adapter).await;
        let config = Self::init_config(size, &surface, &adapter);
        surface.configure(&device, &config);

        // NEW
        let texture_bind_group_layout = device.create_bind_group_layout(
            &Texture::bind_group_layout_descriptor("texture_bind_group_layout"),
        );

        // diffuse texture
        let diffuse_bytes = include_bytes!("happy-tree.png"); // CHANGED!
        let diffuse_texture =
            Texture::from_bytes(&device, &queue, diffuse_bytes, "happy-tree").unwrap(); // CHANGED!
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

        let num_indices = Vertex::INDICES.len() as u32;

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
            num_indices,
            use_color,
            default_texture: diffuse_texture,
            bind_group: diffuse_bind_group,
            texture_bind_group_layout,
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
        let angle = std::f32::consts::PI;
        let new_vertex = Vertex::rotate_vertex(angle, Vertex::VERTICES_PENTAGON);
        // let num_triangles = self.num_vertices - 2;
        // let new_indices = (1u16..num_triangles as u16 + 1)
        //     .into_iter()
        //     .flat_map(|i| vec![i + 1, i, 0])
        //     .collect::<Vec<_>>();
        let new_indices = Vertex::INDICES;

        let cartoon_bytes = include_bytes!("happy-tree-cartoon.png");
        let cartoon_texture = Texture::from_bytes(
            &self.device,
            &self.queue,
            cartoon_bytes,
            "happy-tree-cartoon",
        ).unwrap();

        match self.use_color {
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
                self.bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                    layout: &self.texture_bind_group_layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::TextureView(&self.default_texture.view),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::Sampler(&self.default_texture.sampler),
                        },
                    ],
                    label: Some("diffuse_bind_group"),
                });
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
                self.bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                    layout: &self.texture_bind_group_layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::TextureView(&cartoon_texture.view),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::Sampler(&cartoon_texture.sampler),
                        },
                    ],
                    label: Some("diffuse_bind_group"),
                });
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
        render_pass.set_bind_group(0, &self.bind_group, &[]);
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
