use image::RgbImage;
use wgpu::util::DeviceExt;
use wgpu::*;
use glam::Vec3;
use crate::model::Vertex;

pub struct ImageViewer {
    pub texture: Option<Texture>,
    pub texture_view: Option<TextureView>,
    pub bind_group: Option<BindGroup>,
    pub vertex_buffer: Option<Buffer>,
    pub index_buffer: Option<Buffer>,
    pub index_count: u32,
    pub mode: ImageMode,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ImageMode {
    Texture3D,
    Viewer2D,
}

impl ImageViewer {
    pub fn new() -> Self {
        Self {
            texture: None,
            texture_view: None,
            bind_group: None,
            vertex_buffer: None,
            index_buffer: None,
            index_count: 0,
            mode: ImageMode::Texture3D,
        }
    }

    pub fn load_image(&mut self, device: &Device, queue: &Queue, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let img = image::open(path)?;
        let rgb = img.to_rgb8();
        let (width, height) = rgb.dimensions();

        let size = Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&TextureDescriptor {
            label: Some("Image Texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[],
        });

        let rgba_data: Vec<u8> = rgb
            .pixels()
            .flat_map(|p| [p[0], p[1], p[2], 255])
            .collect();

        queue.write_texture(
            ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
                aspect: TextureAspect::All,
            },
            &rgba_data,
            ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
            size,
        );

        let texture_view = texture.create_view(&TextureViewDescriptor::default());

        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        multisampled: false,
                        view_dimension: TextureViewDimension::D2,
                        sample_type: TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("texture_bind_group_layout"),
        });

        let sampler = device.create_sampler(&SamplerDescriptor {
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            mipmap_filter: FilterMode::Nearest,
            ..Default::default()
        });

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&texture_view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&sampler),
                },
            ],
            label: Some("texture_bind_group"),
        });

        let aspect = width as f32 / height as f32;
        let scale = 2.0;
        let vertices = vec![
            Vertex::new(Vec3::new(-scale * aspect, -scale, 0.0), Vec3::new(0.0, 0.0, 1.0)),
            Vertex::new(Vec3::new(scale * aspect, -scale, 0.0), Vec3::new(0.0, 0.0, 1.0)),
            Vertex::new(Vec3::new(scale * aspect, scale, 0.0), Vec3::new(0.0, 0.0, 1.0)),
            Vertex::new(Vec3::new(-scale * aspect, scale, 0.0), Vec3::new(0.0, 0.0, 1.0)),
        ];

        let indices = vec![0, 1, 2, 2, 3, 0];

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Image Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Image Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: BufferUsages::INDEX,
        });

        self.texture = Some(texture);
        self.texture_view = Some(texture_view);
        self.bind_group = Some(bind_group);
        self.vertex_buffer = Some(vertex_buffer);
        self.index_buffer = Some(index_buffer);
        self.index_count = indices.len() as u32;

        Ok(())
    }

    pub fn toggle_mode(&mut self) {
        self.mode = match self.mode {
            ImageMode::Texture3D => ImageMode::Viewer2D,
            ImageMode::Viewer2D => ImageMode::Texture3D,
        };
    }

    pub fn has_image(&self) -> bool {
        self.texture.is_some()
    }
}

