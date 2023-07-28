## Texture and bind groups
### TextureViews and Samplers
- Textures are images overlaid on a triangle mesh.
```rust,noplayground
// We don't need to configure the texture view much, so let's
// let wgpu define it.
let diffuse_texture_view = diffuse_texture.create_view(&wgpu::TextureViewDescriptor::default());
let diffuse_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
    address_mode_u: wgpu::AddressMode::ClampToEdge,
    address_mode_v: wgpu::AddressMode::ClampToEdge,
    address_mode_w: wgpu::AddressMode::ClampToEdge,
    mag_filter: wgpu::FilterMode::Linear,
    min_filter: wgpu::FilterMode::Nearest,
    mipmap_filter: wgpu::FilterMode::Nearest,
    ..Default::default()
});
```
- `address_mode_*`: determine wat to do if the sampler gets a texture coordinate that's outside the texture itself.
- `mag_filter` and `min_filter`: describe what to do when sample  footprint is smaller or larger than one texel. 
These 2 fields usually work when the mapping in the scene is far from or close tothe camera.
    - `Linear`: Select 2 texels in each dimension, return linear interpolation between their values
    - `Nearest`: Return the value of the texel nearest to the texture coordinates.
- `mipmap`:
```txt
Mipmapping is particularly useful when rendering textures at varying distances 
from the camera. As objects move further away, their textures appear smaller 
on the screen, and using a full-resolution texture can be wasteful in terms of memory 
and processing power. 
Mipmaps provide a precomputed series of progressively smaller versions of the original texture, 
allowing the renderer to select the most appropriate mipmap level based on the size of 
the texture on the screen.
In the context of the WebGPU API, which is a low-level graphics and compute API for the web, 
mipmapping is supported through the use of texture views. A texture view is a subset of 
a texture that provides a specific level of detail, such as a particular mipmap level. 
By creating texture views with different mipmap levels, developers can control the level of 
detail used during rendering, balancing visual quality with performance.
```
![Texture Clamping](../assets/images/texture.png)
### The BindGroup
- A `BingGroup` describes a set of resources and how they can be accessed by a shader.
#### bindgroup_layout
```rust,noplayground
let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        // This should match the filterable field of the
                        // corresponding Texture entry above.
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });
```
- `texture_bind_group_layout` has  2 entries: `sampled texture` at `binding 0` and `sampler` at `binding 1`. (`FRAGMENT`)
- The possible values for `entries` are `NONE`, `VERTEX`, `FRAGMENT`, `COMPUTE`. Most of the time, only `FRAGMENT` is used.

#### bind_group
```rust,noplayground
let diffuse_bind_group = device.create_bind_group(
    &wgpu::BindGroupDescriptor {
        layout: &texture_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&diffuse_texture_view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&diffuse_sampler),
            }
        ],
        label: Some("diffuse_bind_group"),
    }
);
```
- The reason why we seperate `bind_group` from `bind_group_layout` is that 
it allows us to swap out `BindGroup`s on the fly, 
as long as they all share the same `BindGrouplayout`.
- Notes: `position` bufferspace is from `-0.5:0.5` `tex_coords` is from `0.0:1.0`
- `tex_coords` y direction is down. 
```rust
tex_coords_y = 1.0 - (position_y + 0.5)
```
