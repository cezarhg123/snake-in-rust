use glium::backend::Facade;

use crate::vertex::Vertex;

pub struct Drawable {
    vb: glium::VertexBuffer<Vertex>,
    eb: glium::IndexBuffer<u32>
}

impl Drawable {
    pub fn new(display: &dyn Facade, position: (i32, i32), color: (f32, f32, f32)) -> Drawable {
        let vertices: Vec<Vertex> = 
        vec![
            Vertex {
                position: (position.0 + 5, position.1 + 5),
                color
            },

            Vertex {
                position: (position.0 + 5, position.1 + 75),
                color
            },

            Vertex {
                position: (position.0 + 75, position.1 + 5),
                color
            },

            Vertex {
                position: (position.0 + 75, position.1 + 75),
                color
            }
        ];

        let indices: Vec<u32> =
        vec![
            0, 1, 2,
            1, 3, 2
        ];

        Drawable {
            vb: glium::VertexBuffer::new(display, vertices.as_slice()).unwrap(),
            eb: glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, indices.as_slice()).unwrap()
        }
    }

    pub fn get_vb(&self) -> &glium::VertexBuffer<Vertex> {
        &self.vb
    }

    pub fn get_eb(&self) -> &glium::IndexBuffer<u32> {
        &self.eb
    }
}