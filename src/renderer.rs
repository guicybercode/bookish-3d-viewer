use wgpu::util::DeviceExt;
use wgpu::*;
use glam::{Mat4, Vec3};
use crate::model::Model;
use crate::camera::Camera;
use crate::utils;

pub struct Renderer<'window> {
    device: Device,
    queue: Queue,
    surface: Surface<'window>,
    surface_config: SurfaceConfiguration,
    wireframe_pipeline: RenderPipeline,
    flat_pipeline: RenderPipeline,
    image_pipeline: RenderPipeline,
    uniform_bind_group: BindGroup,
    uniform_buffer: Buffer,
    depth_texture: Texture,
    depth_texture_view: TextureView,
    wireframe_mode: bool,
    flat_shading: bool,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    view_proj: [[f32; 4]; 4],
    model: [[f32; 4]; 4],
    color: [f32; 4],
}

impl Uniforms {
    fn new() -> Self {
        Self {
            view_proj: Mat4::IDENTITY.to_cols_array_2d(),
            model: Mat4::IDENTITY.to_cols_array_2d(),
            color: [0.0, 1.0, 0.0, 1.0],
        }
    }
}

impl<'window> Renderer<'window> {
    pub async fn new(window: &'window winit::window::Window) -> Result<Self, Box<dyn std::error::Error>> {
        let size = window.inner_size();
        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            ..Default::default()
        });

        let surface = instance.create_surface(window)?;
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or("Failed to find an appropriate adapter")?;

        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    required_features: Features::empty(),
                    required_limits: Limits::default(),
                    label: None,
                },
                None,
            )
            .await?;

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let surface_config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &surface_config);

        let depth_texture = Self::create_depth_texture(&device, size.width, size.height);
        let depth_texture_view = depth_texture.create_view(&TextureViewDescriptor::default());

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[Uniforms::new()]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let uniform_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
                        ty: BindingType::Buffer {
                            ty: BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
                label: Some("uniform_bind_group_layout"),
            });

        let uniform_bind_group = device.create_bind_group(&BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: Some("uniform_bind_group"),
        });

        let wireframe_pipeline = Self::create_wireframe_pipeline(
            &device,
            &surface_config,
            &uniform_bind_group_layout,
        );
        let flat_pipeline = Self::create_flat_pipeline(
            &device,
            &surface_config,
            &uniform_bind_group_layout,
        );
        let texture_bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
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

        let image_pipeline = Self::create_image_pipeline(
            &device,
            &surface_config,
            &uniform_bind_group_layout,
            &texture_bind_group_layout,
        );

        Ok(Self {
            device,
            queue,
            surface,
            surface_config,
            wireframe_pipeline,
            flat_pipeline,
            image_pipeline,
            uniform_bind_group,
            uniform_buffer,
            depth_texture,
            depth_texture_view,
            wireframe_mode: false,
            flat_shading: true,
        })
    }

    fn create_depth_texture(device: &Device, width: u32, height: u32) -> Texture {
        device.create_texture(&TextureDescriptor {
            size: Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Depth32Float,
            usage: TextureUsages::RENDER_ATTACHMENT,
            label: Some("depth_texture"),
            view_formats: &[],
        })
    }

    fn create_wireframe_pipeline(
        device: &Device,
        config: &SurfaceConfiguration,
        bind_group_layout: &BindGroupLayout,
    ) -> RenderPipeline {
        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Wireframe Shader"),
            source: ShaderSource::Wgsl(include_str!("shaders/wireframe.wgsl").into()),
        });

        let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Wireframe Pipeline Layout"),
            bind_group_layouts: &[bind_group_layout],
            push_constant_ranges: &[],
        });

        device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Wireframe Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[VertexBufferLayout {
                    array_stride: std::mem::size_of::<crate::model::Vertex>() as u64,
                    step_mode: VertexStepMode::Vertex,
                    attributes: &[
                        VertexAttribute {
                            offset: 0,
                            shader_location: 0,
                            format: VertexFormat::Float32x3,
                        },
                        VertexAttribute {
                            offset: std::mem::size_of::<[f32; 3]>() as u64,
                            shader_location: 1,
                            format: VertexFormat::Float32x3,
                        },
                    ],
                }],
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(ColorTargetState {
                    format: config.format,
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::LineList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                polygon_mode: PolygonMode::Line,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(DepthStencilState {
                format: TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: CompareFunction::Less,
                stencil: StencilState::default(),
                bias: DepthBiasState::default(),
            }),
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        })
    }

    fn create_flat_pipeline(
        device: &Device,
        config: &SurfaceConfiguration,
        bind_group_layout: &BindGroupLayout,
    ) -> RenderPipeline {
        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Flat Shader"),
            source: ShaderSource::Wgsl(include_str!("shaders/flat.wgsl").into()),
        });

        let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Flat Pipeline Layout"),
            bind_group_layouts: &[bind_group_layout],
            push_constant_ranges: &[],
        });

        device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Flat Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[VertexBufferLayout {
                    array_stride: std::mem::size_of::<crate::model::Vertex>() as u64,
                    step_mode: VertexStepMode::Vertex,
                    attributes: &[
                        VertexAttribute {
                            offset: 0,
                            shader_location: 0,
                            format: VertexFormat::Float32x3,
                        },
                        VertexAttribute {
                            offset: std::mem::size_of::<[f32; 3]>() as u64,
                            shader_location: 1,
                            format: VertexFormat::Float32x3,
                        },
                    ],
                }],
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(ColorTargetState {
                    format: config.format,
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                polygon_mode: PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(DepthStencilState {
                format: TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: CompareFunction::Less,
                stencil: StencilState::default(),
                bias: DepthBiasState::default(),
            }),
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        })
    }

    fn create_image_pipeline(
        device: &Device,
        config: &SurfaceConfiguration,
        uniform_bind_group_layout: &BindGroupLayout,
        texture_bind_group_layout: &BindGroupLayout,
    ) -> RenderPipeline {
        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Image Shader"),
            source: ShaderSource::Wgsl(include_str!("shaders/image.wgsl").into()),
        });

        let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Image Pipeline Layout"),
            bind_group_layouts: &[uniform_bind_group_layout, texture_bind_group_layout],
            push_constant_ranges: &[],
        });

        device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Image Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[VertexBufferLayout {
                    array_stride: std::mem::size_of::<crate::model::Vertex>() as u64,
                    step_mode: VertexStepMode::Vertex,
                    attributes: &[
                        VertexAttribute {
                            offset: 0,
                            shader_location: 0,
                            format: VertexFormat::Float32x3,
                        },
                        VertexAttribute {
                            offset: std::mem::size_of::<[f32; 3]>() as u64,
                            shader_location: 1,
                            format: VertexFormat::Float32x3,
                        },
                    ],
                }],
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(ColorTargetState {
                    format: config.format,
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(DepthStencilState {
                format: TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: CompareFunction::Less,
                stencil: StencilState::default(),
                bias: DepthBiasState::default(),
            }),
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.surface_config.width = width;
            self.surface_config.height = height;
            self.surface.configure(&self.device, &self.surface_config);
            self.depth_texture = Self::create_depth_texture(&self.device, width, height);
            self.depth_texture_view = self.depth_texture.create_view(&TextureViewDescriptor::default());
        }
    }

    pub fn toggle_wireframe(&mut self) {
        self.wireframe_mode = !self.wireframe_mode;
    }

    pub fn toggle_flat_shading(&mut self) {
        self.flat_shading = !self.flat_shading;
    }

    pub fn is_wireframe(&self) -> bool {
        self.wireframe_mode
    }

    pub fn is_flat_shading(&self) -> bool {
        self.flat_shading
    }

    pub fn render(
        &mut self,
        camera: &Camera,
        model: Option<&Model>,
        image_plane: Option<(&Buffer, &Buffer, u32, &BindGroup)>,
        wireframe_color: u32,
        flat_color: u32,
    ) -> Result<(), SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let model_buffers = if let Some(model) = model {
            let view_proj = camera.get_projection_matrix() * camera.get_view_matrix();
            let model_matrix = Mat4::IDENTITY;

            let color = if self.wireframe_mode {
                utils::color_to_rgba(wireframe_color)
            } else if self.flat_shading {
                utils::color_to_rgba(flat_color)
            } else {
                utils::color_to_rgba(wireframe_color)
            };

            let uniforms = Uniforms {
                view_proj: view_proj.to_cols_array_2d(),
                model: model_matrix.to_cols_array_2d(),
                color,
            };

            self.queue.write_buffer(
                &self.uniform_buffer,
                0,
                bytemuck::cast_slice(&[uniforms]),
            );

            Some((
                self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(&model.vertices),
                    usage: BufferUsages::VERTEX,
                }),
                self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Index Buffer"),
                    contents: bytemuck::cast_slice(&model.indices),
                    usage: BufferUsages::INDEX,
                }),
                model.indices.len() as u32,
            ))
        } else {
            None
        };

        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::BLACK),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                    view: &self.depth_texture_view,
                    depth_ops: Some(Operations {
                        load: LoadOp::Clear(1.0),
                        store: StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            if let Some((ref vertex_buffer, ref index_buffer, index_count)) = model_buffers {
                render_pass.set_pipeline(if self.wireframe_mode {
                    &self.wireframe_pipeline
                } else {
                    &self.flat_pipeline
                });

                render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
                render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                render_pass.set_index_buffer(index_buffer.slice(..), IndexFormat::Uint32);
                render_pass.draw_indexed(0..index_count, 0, 0..1);
            }

            if let Some((vertex_buf, index_buf, index_count, texture_bind_group)) = image_plane {
                let view_proj = camera.get_projection_matrix() * camera.get_view_matrix();
                let model_matrix = Mat4::IDENTITY;
                let uniforms = Uniforms {
                    view_proj: view_proj.to_cols_array_2d(),
                    model: model_matrix.to_cols_array_2d(),
                    color: [1.0, 1.0, 1.0, 1.0],
                };

                self.queue.write_buffer(
                    &self.uniform_buffer,
                    0,
                    bytemuck::cast_slice(&[uniforms]),
                );

                render_pass.set_pipeline(&self.image_pipeline);
                render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
                render_pass.set_bind_group(1, texture_bind_group, &[]);
                render_pass.set_vertex_buffer(0, vertex_buf.slice(..));
                render_pass.set_index_buffer(index_buf.slice(..), IndexFormat::Uint32);
                render_pass.draw_indexed(0..index_count, 0, 0..1);
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn queue(&self) -> &Queue {
        &self.queue
    }

    pub fn size(&self) -> (u32, u32) {
        (self.surface_config.width, self.surface_config.height)
    }
}