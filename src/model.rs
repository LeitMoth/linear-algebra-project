use glam::{Mat4, Vec3};

#[derive(Debug)]
pub struct Model {
    pub name: String,
    pub verts: Vec<Vec3>,
    pub indices: Vec<[usize; 3]>,
    pub transform: Mat4,
}
