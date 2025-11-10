use glam::{Mat4, Vec3};

#[derive(Debug, Clone)]
pub struct Transform {
    pub translation: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            translation: Vec3::ZERO,
            rotation: Vec3::ZERO,
            scale: Vec3::ONE,
        }
    }
}

impl Transform {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn translate(&mut self, delta: Vec3) {
        self.translation += delta;
    }

    pub fn rotate(&mut self, delta: Vec3) {
        self.rotation += delta;
    }

    pub fn scale_by(&mut self, factor: Vec3) {
        self.scale *= factor;
    }

    pub fn to_matrix(&self) -> Mat4 {
        let translation_mat = Mat4::from_translation(self.translation);
        let rotation_x = Mat4::from_rotation_x(self.rotation.x);
        let rotation_y = Mat4::from_rotation_y(self.rotation.y);
        let rotation_z = Mat4::from_rotation_z(self.rotation.z);
        let scale_mat = Mat4::from_scale(self.scale);
        
        translation_mat * rotation_z * rotation_y * rotation_x * scale_mat
    }

    pub fn reset(&mut self) {
        self.translation = Vec3::ZERO;
        self.rotation = Vec3::ZERO;
        self.scale = Vec3::ONE;
    }
}

