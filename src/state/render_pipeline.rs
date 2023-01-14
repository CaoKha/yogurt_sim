use super::buffer;
use super::vertex::Vertex;

pub struct RenderPipeline {
    render_pipeline: wgpu::RenderPipeline,
    // Challenge tutorial Pipeline
    alt_render_pipeline: wgpu::RenderPipeline,
    use_alt: bool,
}

impl RenderPipeline {
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let render_pipeline = init_render_pipeline(&device, &config, &shader, "vs_main", "fs_main");
        let alt_render_pipeline =
            init_render_pipeline(&device, &config, &shader, "vs_main", "fs_main_alt");
        let use_alt = false;

        RenderPipeline {
            render_pipeline,
            alt_render_pipeline,
            use_alt,
        }
    }

    pub fn swap_pipeline(&mut self) {
        self.use_alt = !self.use_alt;
    }

    pub fn add_render_pass(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        color: wgpu::Color,
        vertex_buffer: &buffer::Buffer,
        index_buffer: &Option<buffer::Buffer>,
    ) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(color),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        render_pass.set_pipeline(if self.use_alt {
            &self.alt_render_pipeline
        } else {
            &self.render_pipeline
        });

        render_pass.set_vertex_buffer(0, vertex_buffer.buffer.slice(..));

        if let Some(index_buffer) = index_buffer {
            render_pass.set_index_buffer(index_buffer.buffer.slice(..), wgpu::IndexFormat::Uint16);
            index_buffer.drawn_on(&mut render_pass);
        } else {
            vertex_buffer.drawn_on(&mut render_pass);
        }
    }
}

fn init_render_pipeline(
    device: &wgpu::Device,
    config: &wgpu::SurfaceConfiguration,
    shader: &wgpu::ShaderModule,
    vertex_entry_point: &str,
    fragment_entry_point: &str,
) -> wgpu::RenderPipeline {
    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline layout"),
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: vertex_entry_point,
            buffers: &[Vertex::desc()],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: fragment_entry_point,
            targets: &[Some(wgpu::ColorTargetState {
                format: config.format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
            polygon_mode: wgpu::PolygonMode::Fill,
            // Requires Features::DEPTH_CLIP_CONTROL
            unclipped_depth: false,
            // Requires Features::CONSERVATIVE_RASTERIZATION
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    })
}
