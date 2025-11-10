use glam::Vec3;
use crate::utils;

pub struct ModelInfo {
    pub vertex_count: usize,
    pub face_count: usize,
    pub bounds_min: Vec3,
    pub bounds_max: Vec3,
    pub center: Vec3,
    pub file_path: Option<String>,
    pub file_size: Option<u64>,
}

impl ModelInfo {
    pub fn from_model(model: &crate::model::Model, path: Option<String>) -> Self {
        let vertex_count = model.vertices.len();
        let face_count = model.indices.len() / 3;
        let (min, max) = utils::calculate_bounds(
            &model.vertices.iter().map(|v| v.position_vec3()).collect::<Vec<_>>()
        );
        
        let file_size = path.as_ref()
            .and_then(|p| std::fs::metadata(p).ok())
            .map(|m| m.len());
        
        Self {
            vertex_count,
            face_count,
            bounds_min: min,
            bounds_max: max,
            center: model.center,
            file_path: path,
            file_size,
        }
    }

    pub fn format_info(&self) -> String {
        let file_info = if let Some(ref path) = self.file_path {
            let name = utils::get_file_name(path);
            let size_str = self.file_size
                .map(|s| format!(" ({})", utils::format_file_size(s)))
                .unwrap_or_default();
            format!("File: {}{}", name, size_str)
        } else {
            String::from("File: Unknown")
        };
        
        format!(
            "{}\nVertices: {}\nFaces: {}\nCenter: ({:.2}, {:.2}, {:.2})\nBounds: ({:.2}, {:.2}, {:.2}) to ({:.2}, {:.2}, {:.2})",
            file_info,
            self.vertex_count,
            self.face_count,
            self.center.x, self.center.y, self.center.z,
            self.bounds_min.x, self.bounds_min.y, self.bounds_min.z,
            self.bounds_max.x, self.bounds_max.y, self.bounds_max.z,
        )
    }
}

