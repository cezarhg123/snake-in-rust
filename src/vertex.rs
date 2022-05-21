#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub position: (i32, i32),
    pub color: (f32, f32, f32)
}
glium::implement_vertex!(Vertex, position, color);

impl Vertex {
    ///returns a empty vertex
    pub fn new() -> Vertex {
        Vertex {
            position: (0, 0),
            color: (0.0, 0.0, 0.0)
        }
    }
}