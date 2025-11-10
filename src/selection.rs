use glam::Vec3;

#[derive(Debug, Clone)]
pub struct Selection {
    pub selected_vertices: Vec<usize>,
    pub selected_faces: Vec<usize>,
    pub mode: SelectionMode,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SelectionMode {
    None,
    Vertex,
    Face,
    Object,
}

impl Selection {
    pub fn new() -> Self {
        Self {
            selected_vertices: Vec::new(),
            selected_faces: Vec::new(),
            mode: SelectionMode::None,
        }
    }

    pub fn clear(&mut self) {
        self.selected_vertices.clear();
        self.selected_faces.clear();
        self.mode = SelectionMode::None;
    }

    pub fn select_vertex(&mut self, index: usize) {
        if !self.selected_vertices.contains(&index) {
            self.selected_vertices.push(index);
        }
        self.mode = SelectionMode::Vertex;
    }

    pub fn select_face(&mut self, index: usize) {
        if !self.selected_faces.contains(&index) {
            self.selected_faces.push(index);
        }
        self.mode = SelectionMode::Face;
    }

    pub fn deselect_vertex(&mut self, index: usize) {
        self.selected_vertices.retain(|&i| i != index);
        if self.selected_vertices.is_empty() && self.selected_faces.is_empty() {
            self.mode = SelectionMode::None;
        }
    }

    pub fn deselect_face(&mut self, index: usize) {
        self.selected_faces.retain(|&i| i != index);
        if self.selected_vertices.is_empty() && self.selected_faces.is_empty() {
            self.mode = SelectionMode::None;
        }
    }

    pub fn is_selected(&self) -> bool {
        !self.selected_vertices.is_empty() || !self.selected_faces.is_empty()
    }

    pub fn ray_intersect_vertex(
        &self,
        ray_origin: Vec3,
        ray_dir: Vec3,
        vertices: &[(f32, f32, f32)],
        threshold: f32,
    ) -> Option<usize> {
        let mut closest_dist = f32::MAX;
        let mut closest_idx = None;

        for (i, vertex) in vertices.iter().enumerate() {
            let v = Vec3::new(vertex.0, vertex.1, vertex.2);
            let to_vertex = v - ray_origin;
            let projection = to_vertex.dot(ray_dir);
            
            if projection > 0.0 {
                let point_on_ray = ray_origin + ray_dir * projection;
                let dist = (point_on_ray - v).length();
                
                if dist < threshold && dist < closest_dist {
                    closest_dist = dist;
                    closest_idx = Some(i);
                }
            }
        }

        closest_idx
    }
}

