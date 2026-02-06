
use nalgebra::Vector3;
use rand::distr::{Distribution, StandardUniform};
use rand::Rng;
#[derive(Debug)]
pub struct Triangle {
    vertices: [Vector3<f32>; 3],
}
pub struct Point {
    pub position: Vector3<f32>, 
    pub color: Vector3<f32>, 
}
impl Distribution<Triangle> for StandardUniform {

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Triangle {
        let coords: [[f32; 3]; 3] = rng.random();
        let x = Vector3::from(coords[0]);
        let y = Vector3::from(coords[1]);
        let z = Vector3::from(coords[2]);
        match Triangle::new(x, y, z) {
            Some(t) => t,
            None => { self.sample(rng) }            
        }
    }
}
impl Triangle {
    pub fn new(v0: Vector3<f32>, v1: Vector3<f32>, v2: Vector3<f32>) -> Option<Self> {
        let ab = v1 - v0;
        let ac = v2 - v0;
        if ab.cross(&ac).norm() == 0.0 { None }
        else { Some(Triangle{ vertices: [v0, v1, v2] }) }
    }
}
