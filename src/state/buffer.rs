use wgpu::util::DeviceExt;

pub fn init_vertex_buffer(
    device: &wgpu::Device,
    vertices: &[super::vertex::Vertex],
) -> wgpu::Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(vertices),
        usage: wgpu::BufferUsages::VERTEX,
    })
}
