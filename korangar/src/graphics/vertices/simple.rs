use bytemuck::{Pod, Zeroable};
use cgmath::Point3;
use wgpu::{VertexAttribute, VertexBufferLayout, VertexStepMode, vertex_attr_array};

#[repr(C)]
#[derive(Default, Debug, Clone, Copy, Zeroable, Pod)]
pub struct SimpleVertex {
    pub position: [f32; 3],
}

impl SimpleVertex {
    pub fn new(position: Point3<f32>) -> Self {
        Self { position: position.into() }
    }

    pub fn buffer_layout() -> VertexBufferLayout<'static> {
        static ATTRIBUTES: &[VertexAttribute] = &vertex_attr_array!(
            0 => Float32x3,
        );

        VertexBufferLayout {
            array_stride: size_of::<Self>() as _,
            step_mode: VertexStepMode::Vertex,
            attributes: ATTRIBUTES,
        }
    }
}
