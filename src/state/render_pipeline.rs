
use super::buffer;
use super::vertex::Vertex;

pub struct RenderPipeline {
    render_pipeline: wgpu::RenderPipeline,
}

impl RenderPipeline {
    pub fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        shader: wgpu::ShaderModule
        // bind_group_layouts: &[&wgpu::BindGroupLayout],
    ) -> Self {

        let render_pipeline = init_render_pipeline(
            &device,
            &config,
            &shader,
            "vs_main",
            "fs_main",
            // bind_group_layouts,
        );



        RenderPipeline { render_pipeline }
    }

    pub fn add_render_pass(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        color: wgpu::Color,
        vertex_buffer: &buffer::Buffer,
        index_buffer: &Option<buffer::Buffer>,
        num_vertices: u32
        // diffuse_bind_group: &wgpu::BindGroup,
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

        render_pass.set_pipeline(&self.render_pipeline);

        // render_pass.set_bind_group(0, diffuse_bind_group, &[]);

        // vertex_buffer.attach_to(&mut render_pass);
        // if let Some(index_buffer) = index_buffer {
        //     index_buffer.attach_to(&mut render_pass);
        // }

        // if let Some(index_buffer) = index_buffer {
        //     index_buffer.drawn_on(&mut render_pass);
        // } else {
        //     vertex_buffer.drawn_on(&mut render_pass);
        // }
        render_pass.set_vertex_buffer(0, (vertex_buffer.buffer).slice(..));
        render_pass.draw(0..num_vertices, 0..1);
    }
}

fn init_render_pipeline(
    device: &wgpu::Device,
    config: &wgpu::SurfaceConfiguration,
    shader: &wgpu::ShaderModule,
    vertex_entry_point: &str,
    fragment_entry_point: &str,
    // bg_layouts: &[&wgpu::BindGroupLayout],
) -> wgpu::RenderPipeline {
    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline layout"),
        // bind_group_layouts: bg_layouts,
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: vertex_entry_point,
            // buffers: &[Vertex::desc()],
            buffers: &[Vertex::desc()],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: fragment_entry_point,
            targets: &[Some(wgpu::ColorTargetState {
                format: config.format,
                blend: Some(wgpu::BlendState {
                    color: wgpu::BlendComponent::REPLACE,
                    alpha: wgpu::BlendComponent::REPLACE
                }),
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
