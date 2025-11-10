use glam::{Mat4, Vec3};

pub struct Camera {
    pub position: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
    pub distance: f32,
    pub rotation_x: f32,
    pub rotation_y: f32,
    pub pan_x: f32,
    pub pan_y: f32,
}

impl Camera {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            position: Vec3::new(0.0, 0.0, 5.0),
            target: Vec3::ZERO,
            up: Vec3::Y,
            fov: 45.0_f32.to_radians(),
            aspect: width / height,
            near: 0.1,
            far: 1000.0,
            distance: 5.0,
            rotation_x: 0.0,
            rotation_y: 0.0,
            pan_x: 0.0,
            pan_y: 0.0,
        }
    }

    pub fn update_aspect(&mut self, width: f32, height: f32) {
        self.aspect = width / height;
    }

    pub fn rotate(&mut self, delta_x: f32, delta_y: f32) {
        self.rotation_y += delta_x * 0.01;
        self.rotation_x += delta_y * 0.01;
        self.rotation_x = self.rotation_x.clamp(-1.57, 1.57);
    }

    pub fn zoom(&mut self, delta: f32) {
        self.distance += delta * 0.1;
        self.distance = self.distance.clamp(0.5, 50.0);
    }

    pub fn pan(&mut self, delta_x: f32, delta_y: f32) {
        let right = self.get_right();
        let up = self.get_up();
        let pan_speed = self.distance * 0.001;
        self.pan_x += right.x * delta_x * pan_speed;
        self.pan_y += right.y * delta_x * pan_speed;
        self.pan_x += up.x * delta_y * pan_speed;
        self.pan_y += up.y * delta_y * pan_speed;
    }

    pub fn reset(&mut self) {
        self.position = Vec3::new(0.0, 0.0, 5.0);
        self.target = Vec3::ZERO;
        self.distance = 5.0;
        self.rotation_x = 0.0;
        self.rotation_y = 0.0;
        self.pan_x = 0.0;
        self.pan_y = 0.0;
    }

    fn get_right(&self) -> Vec3 {
        let forward = (self.target - self.position).normalize();
        forward.cross(self.up).normalize()
    }

    fn get_up(&self) -> Vec3 {
        let forward = (self.target - self.position).normalize();
        let right = forward.cross(self.up).normalize();
        right.cross(forward).normalize()
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        let cos_x = self.rotation_x.cos();
        let sin_x = self.rotation_x.sin();
        let cos_y = self.rotation_y.cos();
        let sin_y = self.rotation_y.sin();

        let eye = Vec3::new(
            self.distance * cos_x * sin_y + self.pan_x,
            self.distance * sin_x + self.pan_y,
            self.distance * cos_x * cos_y,
        );

        Mat4::look_at_rh(eye, self.target, self.up)
    }

    pub fn get_projection_matrix(&self) -> Mat4 {
        Mat4::perspective_rh(self.fov, self.aspect, self.near, self.far)
    }
}

