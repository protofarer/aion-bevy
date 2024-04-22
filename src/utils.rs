use std::f32::consts::PI;

use bevy::math::{EulerRot, Quat, Vec2, Vec3};

use crate::{Speed, DEFAULT_HEADING};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Heading(pub f32); // degrees

impl Heading {
    pub fn to_vec3(&self) -> Vec3 {
        let angle_radians = self.0.to_radians();
        let x = angle_radians.cos();
        let y = angle_radians.sin();
        Vec3::new(x, y, 0.)
    }
    pub fn to_vec2(&self) -> Vec2 {
        let angle_radians = self.0.to_radians();
        let x = angle_radians.cos();
        let y = angle_radians.sin();
        Vec2::new(x, y)
    }
    pub fn linvel(&self, speed: Speed) -> Vec2 {
        self.to_vec2() * speed
    }
}

impl Default for Heading {
    fn default() -> Self {
        DEFAULT_HEADING.clone()
    }
}

impl Into<Quat> for Heading {
    fn into(self) -> Quat {
        let angle_radians = self.0 * PI / 180.;
        Quat::from_rotation_z(angle_radians)
    }
}

impl From<Quat> for Heading {
    fn from(quat: Quat) -> Self {
        let (z_rot, x_rot, y_rot) = quat.to_euler(EulerRot::ZXY);
        Heading(z_rot * 180. / PI)
    }
}
