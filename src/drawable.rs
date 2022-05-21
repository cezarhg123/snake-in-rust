use glium::backend::Facade;

use crate::vertex::Vertex;

pub struct Drawable {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub vb: Option<glium::VertexBuffer<Vertex>>,
    pub eb: Option<glium::IndexBuffer<u32>>
}

impl Drawable {
    pub fn gen_buffers(&mut self, display: &dyn Facade) {
        self.vb = Some(glium::VertexBuffer::new(display, self.vertices.as_slice()).unwrap());
        self.eb = Some(glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, self.indices.as_slice()).unwrap());
    }

    pub fn get_vb(&self) -> &glium::VertexBuffer<Vertex> {
        self.vb.as_ref().unwrap()
    }

    pub fn get_eb(&self) -> &glium::IndexBuffer<u32> {
        self.eb.as_ref().unwrap()
    }
}