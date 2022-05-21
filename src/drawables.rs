use crate::vertex::Vertex;
struct Quad {
    pub position: (i32, i32),
    pub local_vertices: [Vertex; 4],
    global_vertices: [Vertex; 4],
    vertex_buffer: glium::vertex::VertexBuffer<Vertex>,
    indices: Vec<u32>
}

impl Quad {
    pub fn transform_to_global_vertices(&mut self) {
        self.global_vertices[0].position.0 = self.local_vertices[0].position.0 - self.position.0;
        self.global_vertices[0].position.1 = self.local_vertices[0].position.1 - self.position.1;
    }
}