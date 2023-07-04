use wgpu::util::DeviceExt;

pub struct Buffer {
    pub buffer: wgpu::Buffer,
    size: u32,
}

impl Buffer {
    pub fn new<T: bytemuck::NoUninit>(
        device: &wgpu::Device,
        data: &[T],
        b_usage: wgpu::BufferUsages,
        name: &str,
    ) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(name),
            contents: bytemuck::cast_slice(data),
            usage: b_usage,
        });

        let size = data.len() as u32;

        Self { buffer, size }
    }

    pub fn drawn_on(&self, render_pass: &mut wgpu::RenderPass) {
        let usage = self.buffer.usage();
        match usage {
            wgpu::BufferUsages::VERTEX => {
                render_pass.draw(0..self.size, 0..1);
            }
            wgpu::BufferUsages::INDEX => {
                render_pass.draw_indexed(0..self.size, 0, 0..1);
            }
            _ => {
                format!("Unimplemented drawn_on for usage: {:?}", usage);
            }
        };
    }

    pub fn attach_to<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        let usage = self.buffer.usage();
        match usage {
            wgpu::BufferUsages::VERTEX => {
                render_pass.set_vertex_buffer(0, self.buffer.slice(..));
            }
            wgpu::BufferUsages::INDEX => {
                render_pass.set_index_buffer(self.buffer.slice(..), wgpu::IndexFormat::Uint16);
            }
            _ => {
                format!("Unimplemented attach_to for usage: {:?}", usage);
            }
        };
    }
}
