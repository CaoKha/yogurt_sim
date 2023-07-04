# Shader
## What is a shader
- vs_main() entry for vertex shader, fs_main() entry for fragment shader
- `@builtin(position)` -> position in framebuffer space 
- framebuffer coordinates address the pixels in the framebuffer:
    * A coordinate that start from top-left (x to the right, y down) 
    * Related to `Render Passes` and `Rasterization`
- `clip_position` (analogous to GLSL `gl_Position`)
- clip coordinates is vec4
- `var` is mut but statically type
- `let` is not mut but dynamically type
- `location(0)` tells WGPU to store the return value vec4 in the first color target.
## How to use shader
- create a shader module,  `ShaderModuleDescriptor` has `label` and `source` fields. `source` is `wgpu::ShaderSource::Wgsl(smart_pointer_to_wgsl_file)` 
- create a pipeline layout
```rust
fn init_render_pipeline(
    device: &wgpu::Device,
    config: &wgpu::SurfaceConfiguration,
    shader: &wgpu::ShaderModule,
    vertex_entry_point: &str,
    fragment_entry_point: &str,
    bg_layouts: &[&wgpu::BindGroupLayout],
) -> wgpu::RenderPipeline {
    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline layout"),
        bind_group_layouts: bg_layouts,
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
```
- `buffers` tells `wgpu` type of vertices
- `targets` tells `wgpu` set up color of the output (`color_state`)
- `primitive` interpret vertices, converting them into triangles
    * `PrimitiveTopology::TriangleList`: 3 vertices = 1 triangle
    * `FrontFace::Ccw`: triangle faceing forward if vertices in counter-clockwise direction
    *  Triangles not facing forward are "culled" -> not included in the render (`CullMode::Back`)
- `depth_stencil`: depth/stencil buffer
- `count`: -> multisampling
- `mask`: which sample is active -> !0 (bitwise of 0b0000 ... (u64) -> 0b1111 ... = u64::MAX {2<sup>64</sup> -1})

    
