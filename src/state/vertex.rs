#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}
// pub struct Vertex {
//     pub position: [f32; 3],
//     pub color: [f32; 3],
// }

impl Vertex {
    #[rustfmt::skip]
    pub const VERTICES_PENTAGON: &[Vertex] = &[
        Vertex { position: [-0.0868241, 0.49240386, 0.0], tex_coords: [0.4131759, 0.00759614], }, // A
        Vertex { position: [-0.49513406, 0.06958647, 0.0], tex_coords: [0.0048659444, 0.43041354], }, // B
        Vertex { position: [-0.21918549, -0.44939706, 0.0], tex_coords: [0.28081453, 0.949397], }, // C
        Vertex { position: [0.35966998, -0.3473291, 0.0], tex_coords: [0.85967, 0.84732914], }, // D
        Vertex { position: [0.44147372, 0.2347359, 0.0], tex_coords: [0.9414737, 0.2652641], }, // E
    ];

    // #[rustfmt::skip]
    // pub const VERTICES_ROT_90: &[Vertex] = &[
    //     Vertex { position: [-0.0868241, 0.49240386, 0.0], tex_coords: [0.00759614, 0.4131759] }, // A
    //     Vertex { position: [-0.49513406, 0.06958647, 0.0], tex_coords: [0.43041354, 0.0048659444] }, // B
    //     Vertex { position: [-0.21918549, -0.44939706, 0.0], tex_coords: [0.949397, 0.28081453] }, // C
    //     Vertex { position: [0.35966998, -0.3473291, 0.0], tex_coords: [0.84732914, 0.85967] }, // D
    //     Vertex { position: [0.44147372, 0.2347359, 0.0], tex_coords: [0.2652641, 0.9414737] }, // E
    // ];

    // #[rustfmt::skip]
    // pub const VERTICES_PENTAGON: &[Vertex] = &[
    // Vertex { position: [-0.0868241, 0.49240386, 0.0], color: [0.5, 0.0, 0.5] }, // A
    // Vertex { position: [-0.49513406, 0.06958647, 0.0], color: [0.5, 0.0, 0.5] }, // B
    // Vertex { position: [-0.21918549, -0.44939706, 0.0], color: [0.5, 0.0, 0.5] }, // C
    // Vertex { position: [0.35966998, -0.3473291, 0.0], color: [0.5, 0.0, 0.5] }, // D
    // Vertex { position: [0.44147372, 0.2347359, 0.0], color: [0.5, 0.0, 0.5] }, // E
    // ];

    // #[rustfmt::skip]
    // pub const VERTICES_TRIANGLE_DIF_COLOR: &[Vertex] = &[
    //     Vertex { position: [-0.568241, 0.49240386, 0.0], color: [1.0, 0.0, 0.5] }, // A
    //     Vertex { position: [-0.49513406, 0.6958647, 0.0], color: [0.5, 1.0, 0.5] }, // B
    //     Vertex { position: [-0.21918549, -0.44939706, 0.0], color: [0.5, 0.0, 1.0] }, // C
    //     Vertex { position: [0.35966998, -0.3473291, 0.0], color: [0.1, 0.0, 0.5] }, // D
    //     Vertex { position: [0.44147372, 0.2347359, 0.0], color: [0.5, 0.1, 0.5] }, // E
    //     Vertex { position: [0.54147372, 0.5347359, 0.0], color: [0.5, 0.1, 0.5] }, // E
    // ];

    // #[rustfmt::skip]
    // pub const VERTICES_STAR: &[Vertex] = &[
    // Vertex { position: [0.0, 0.5, 0.0], color: [1.0, 0.0, 0.5] }, // A
    // Vertex { position: [0.15, 0.15, 0.0], color: [0.5, 1.0, 0.5] }, // B
    // Vertex { position: [0.5, 0.0, 0.0], color: [0.5, 1.0, 0.5] }, // C
    // Vertex { position: [0.25, -0.25, 0.0], color: [0.5, 0.0, 0.5] }, // D
    // Vertex { position: [0.3, -0.5, 0.0], color: [0.5, 1.0, 0.5] }, // E
    // Vertex { position: [0.0, -0.25, 0.0], color: [0.5, 1.0, 0.5] }, // F
    // Vertex { position: [-0.3, -0.5, 0.0], color: [0.5, 0.0, 0.5] }, // G
    // Vertex { position: [-0.25, -0.25, 0.0], color: [0.5, 0.0, 0.5] }, // H
    // Vertex { position: [-0.5, 0.0, 0.0], color: [0.5, 1.0, 0.5] }, // I
    // Vertex { position: [-0.15, 0.15, 0.0], color: [0.5, 1.0, 0.5] }, // J
    // ];

    #[rustfmt::skip]
    pub const INDICES: &[u16] = &[
    0, 1, 4,
    1, 2, 4,
    2, 3, 4,
    ];

    // #[rustfmt::skip]
    // pub const INDICES_STAR: &[u16] = &[
    // 0, 9, 1,
    // 1, 3, 2,
    // 3, 5, 4,
    // 7, 6, 5,
    // 9, 8, 7,
    // 1, 9, 3,
    // 9, 5, 3,
    // 9, 7, 5,
    // ];

    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            // attributes: &[
            //     wgpu::VertexAttribute {
            //         offset: 0,
            //         shader_location: 0,
            //         format: wgpu::VertexFormat::Float32x3,
            //     },
            //     wgpu::VertexAttribute {
            //         offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
            //         shader_location: 1,
            //         format: wgpu::VertexFormat::Float32x2,
            //     },
            // ],
            attributes: &Self::ATTRIBS,
        }
    }

    pub fn rotate_vertex(angle: f32, vertex_arr: &[Vertex]) -> Vec<Vertex> {
        let pivot_vertex = [0.0, 0.0];
        let num_vertices = vertex_arr.len() as usize;
        let challenge_verts = (0..num_vertices)
            .map(|i| {
                let new_pos_x = angle.cos() * (vertex_arr[i].position[0] - pivot_vertex[0])
                    + angle.sin() * (vertex_arr[i].position[1] - pivot_vertex[1]) + pivot_vertex[0];
                let new_pos_y = angle.cos() * (vertex_arr[i].position[1] - pivot_vertex[1])  
                    - angle.sin() * (vertex_arr[i].position[0] - pivot_vertex[0]) + pivot_vertex[1];
                let new_texture_x = new_pos_x + 0.5;
                let new_texture_y = 0.5 - new_pos_y;

                Vertex {
                    position: [
                        new_pos_x,
                        new_pos_y,
                        0.0,
                    ],
                    tex_coords: [new_texture_y, new_texture_x],
                    // tex_coords: [vertex_arr[i].tex_coords[0], vertex_arr[i].tex_coords[1]],
                }
            })
            .collect::<Vec<_>>();
        challenge_verts
    }
}
