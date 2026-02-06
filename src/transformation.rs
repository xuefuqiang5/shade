use std::convert::identity;

use crate::triangle::{Triangle};
use nalgebra::{Vector3, Vector4, Matrix4, Rotation3, Point3};
struct Transformation {
    position: Vector3<f32>,
    rotation: Vector3<f32>, 
    scale: Vector3<f32>, 
}
impl Transformation {
    pub fn new() {

    }
    pub fn get_view_matrix(eye_pos: &Point3<f32>, target: &Point3<f32>) -> Matrix4<f32> {
        let up = Vector3::y();
        Matrix4::look_at_rh(eye_pos, target, &up)
    }
    pub fn get_model_matrix(&self) -> Matrix4<f32> {
        let translation = Matrix4::new_translation(&self.position);
        let  rotation = Rotation3::from_euler_angles(
            self.rotation.x,
            self.rotation.y,
            self.rotation.z,
        ).to_homogeneous(); 
        let scale = Matrix4::new_nonuniform_scaling(&self.scale);
        translation * rotation * scale
    }
    pub fn get_projecton_matrix() {

    }
}
