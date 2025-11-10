use glam::{Vec3, Vec4};

pub fn color_to_rgba(color: u32) -> [f32; 4] {
    let r = ((color >> 16) & 0xFF) as f32 / 255.0;
    let g = ((color >> 8) & 0xFF) as f32 / 255.0;
    let b = (color & 0xFF) as f32 / 255.0;
    [r, g, b, 1.0]
}

pub fn normalize_vec3(v: Vec3) -> Vec3 {
    let len = v.length();
    if len > 0.0 {
        v / len
    } else {
        Vec3::ZERO
    }
}

pub fn calculate_center(vertices: &[Vec3]) -> Vec3 {
    if vertices.is_empty() {
        return Vec3::ZERO;
    }
    let sum: Vec3 = vertices.iter().sum();
    sum / vertices.len() as f32
}

pub fn calculate_bounds(vertices: &[Vec3]) -> (Vec3, Vec3) {
    if vertices.is_empty() {
        return (Vec3::ZERO, Vec3::ZERO);
    }
    let mut min = Vec3::new(f32::MAX, f32::MAX, f32::MAX);
    let mut max = Vec3::new(f32::MIN, f32::MIN, f32::MIN);
    for v in vertices {
        min = min.min(*v);
        max = max.max(*v);
    }
    (min, max)
}

pub fn scale_to_fit(vertices: &mut [Vec3], target_size: f32) {
    let (min, max) = calculate_bounds(vertices);
    let size = (max - min).max_element();
    if size > 0.0 {
        let scale = target_size / size;
        for v in vertices.iter_mut() {
            *v *= scale;
        }
    }
}

