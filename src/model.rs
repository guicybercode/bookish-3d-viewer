use glam::Vec3;
use std::fs::File;
use std::io::{BufReader, Cursor};
use crate::utils;
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}

unsafe impl Pod for Vertex {}
unsafe impl Zeroable for Vertex {}

impl Vertex {
    pub fn new(position: Vec3, normal: Vec3) -> Self {
        Self {
            position: position.to_array(),
            normal: normal.to_array(),
        }
    }

    pub fn position_vec3(&self) -> Vec3 {
        Vec3::from_array(self.position)
    }

    pub fn normal_vec3(&self) -> Vec3 {
        Vec3::from_array(self.normal)
    }
}

#[derive(Debug)]
pub struct Model {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub center: Vec3,
}

impl Model {
    pub fn from_obj(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let (models, _materials) = tobj::load_obj_buf(
            &mut reader,
            &tobj::LoadOptions {
                triangulate: true,
                single_index: true,
                ..Default::default()
            },
            |_| Err(tobj::LoadError::GenericFailure),
        )?;

        Self::from_tobj_models(models)
    }

    pub fn from_obj_bytes(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let mut reader = BufReader::new(Cursor::new(data));
        let (models, _materials) = tobj::load_obj_buf(
            &mut reader,
            &tobj::LoadOptions {
                triangulate: true,
                single_index: true,
                ..Default::default()
            },
            |_| Err(tobj::LoadError::GenericFailure),
        )?;

        Self::from_tobj_models(models)
    }

    fn from_tobj_models(models: Vec<tobj::Model>) -> Result<Self, Box<dyn std::error::Error>> {

        if models.is_empty() {
            return Err("No models found in OBJ file".into());
        }

        let mesh = &models[0].mesh;

        let mut vertices = Vec::new();

        let positions = &mesh.positions;
        let normals = &mesh.normals;

        for i in 0..positions.len() / 3 {
            let pos = Vec3::new(
                positions[i * 3],
                positions[i * 3 + 1],
                positions[i * 3 + 2],
            );

            let normal = if i * 3 + 2 < normals.len() {
                Vec3::new(
                    normals[i * 3],
                    normals[i * 3 + 1],
                    normals[i * 3 + 2],
                )
            } else {
                Vec3::ZERO
            };

            vertices.push(Vertex::new(pos, normal));
        }

        let indices = mesh.indices.clone();

        if vertices.is_empty() {
            return Err("No vertices found in OBJ file".into());
        }

        let positions_vec: Vec<Vec3> = vertices.iter().map(|v| v.position_vec3()).collect();
        let center = utils::calculate_center(&positions_vec);
        let mut positions_mut: Vec<Vec3> = vertices.iter().map(|v| v.position_vec3()).collect();
        utils::scale_to_fit(&mut positions_mut, 2.0);
        for (i, pos) in positions_mut.iter().enumerate() {
            vertices[i] = Vertex::new(*pos, vertices[i].normal_vec3());
        }

        for vertex in &mut vertices {
            let normal = vertex.normal_vec3();
            if normal.length_squared() < 0.001 {
                *vertex = Vertex::new(vertex.position_vec3(), Vec3::ZERO);
            }
        }

        Ok(Model {
            vertices,
            indices,
            center,
        })
    }

    pub fn calculate_normals(&mut self) {
        for i in (0..self.indices.len()).step_by(3) {
            if i + 2 < self.indices.len() {
                let i0 = self.indices[i] as usize;
                let i1 = self.indices[i + 1] as usize;
                let i2 = self.indices[i + 2] as usize;

                if i0 < self.vertices.len() && i1 < self.vertices.len() && i2 < self.vertices.len() {
                    let v0 = self.vertices[i0].position_vec3();
                    let v1 = self.vertices[i1].position_vec3();
                    let v2 = self.vertices[i2].position_vec3();

                    let edge1 = v1 - v0;
                    let edge2 = v2 - v0;
                    let normal = edge1.cross(edge2).normalize();

                    let n0 = self.vertices[i0].normal_vec3() + normal;
                    let n1 = self.vertices[i1].normal_vec3() + normal;
                    let n2 = self.vertices[i2].normal_vec3() + normal;

                    self.vertices[i0] = Vertex::new(self.vertices[i0].position_vec3(), n0);
                    self.vertices[i1] = Vertex::new(self.vertices[i1].position_vec3(), n1);
                    self.vertices[i2] = Vertex::new(self.vertices[i2].position_vec3(), n2);
                }
            }
        }

        for vertex in &mut self.vertices {
            let normal = vertex.normal_vec3().normalize();
            *vertex = Vertex::new(vertex.position_vec3(), normal);
        }
    }
}

