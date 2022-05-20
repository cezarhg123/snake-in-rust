#[derive(Clone, Copy)]
pub struct Vertex {
    position: [i32; 2],
    color: [f32; 3]
}
glium::implement_vertex!(Vertex, position, color);

impl Vertex {
    ///returns a empty vertex
    pub fn new() -> Vertex {
        Vertex {
            position: [0, 0],
            color: [0.0, 0.0, 0.0]
        }
    }
}