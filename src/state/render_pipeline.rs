use super::init_utils::init_render_pipeline;

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
            init_render_pipeline(&device, &config, &shader, "vs_main", "fs_main_second");
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
        render_pass.draw(0..3, 0..1);
    }
}
