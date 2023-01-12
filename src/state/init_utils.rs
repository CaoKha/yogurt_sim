use wgpu::util::DeviceExt;

pub async fn init_surface(window: &winit::window::Window) -> (wgpu::Surface, wgpu::Adapter) {
    // the instance is a handle to our GPU
    // Backends::all => Vulkan + Metal + DX12 + Browser WebGPUÒ
    let instance = wgpu::Instance::new(wgpu::Backends::all());
    let surface = unsafe { instance.create_surface(window) };
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
        .await
        .unwrap();
    (surface, adapter)
}

pub async fn init_device(adapter: &wgpu::Adapter) -> (wgpu::Device, wgpu::Queue) {
    adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web we'll have to disable some.
                limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                label: None,
            },
            None,
        )
        .await
        .unwrap()
}

pub fn init_config(
    size: winit::dpi::PhysicalSize<u32>,
    surface: &wgpu::Surface,
    adapter: &wgpu::Adapter,
) -> wgpu::SurfaceConfiguration {
    wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface.get_supported_formats(&adapter)[0],
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: wgpu::CompositeAlphaMode::Auto,
    }
}

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
