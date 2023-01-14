use wgpu::util::DeviceExt;

pub struct Buffer {
    pub buffer: wgpu::Buffer,
    usage: wgpu::BufferUsages,
    size: u32,
}

impl Buffer {
    pub fn new<T: bytemuck::NoUninit>(
        device: &wgpu::Device,
        data: &[T],
        usage: wgpu::BufferUsages,
        name: &str,
    ) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(name),
            contents: bytemuck::cast_slice(data),
            usage: usage,
        });

        let size = data.len() as u32;

        Self {
            buffer,
            usage,
            size,
        }
    }

    pub fn drawn_on(&self, render_pass: &mut wgpu::RenderPass) {
        match self.usage {
            wgpu::BufferUsages::VERTEX => {
                render_pass.draw(0..self.size, 0..1);
            }
            wgpu::BufferUsages::INDEX => {
                render_pass.draw_indexed(0..self.size, 0, 0..1);
            }
            _ => {
                format!("Unimplemented usage: {:?}", self.usage);
            }
        };
    }
}
