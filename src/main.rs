mod camera;
mod model;
mod renderer;
mod image_viewer;
mod menu;
mod utils;
mod config;
mod model_info;
mod error;

use std::rc::Rc;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    keyboard::{Key, NamedKey},
};
use camera::Camera;
use model::Model;
use renderer::Renderer;
use image_viewer::ImageViewer;
use menu::Menu;
use config::Config;
use model_info::ModelInfo;

struct App {
    camera: Camera,
    model: Option<Model>,
    model_info: Option<ModelInfo>,
    image_viewer: ImageViewer,
    menu: Menu,
    config: Config,
    mouse_pressed: bool,
    right_mouse_pressed: bool,
    last_mouse_pos: (f64, f64),
    show_info: bool,
}

impl App {
    fn new() -> Self {
        Self {
            camera: Camera::new(800.0, 600.0),
            model: None,
            model_info: None,
            image_viewer: ImageViewer::new(),
            menu: Menu::new(),
            config: Config::load(),
            mouse_pressed: false,
            right_mouse_pressed: false,
            last_mouse_pos: (0.0, 0.0),
            show_info: false,
        }
    }

    fn handle_keyboard(&mut self, key: &Key) {
        match key {
            Key::Character(ref c) if c == "r" || c == "R" => {
                self.camera.reset();
            }
            Key::Named(NamedKey::ArrowUp) => {
                self.camera.rotate(0.0, -0.1);
            }
            Key::Named(NamedKey::ArrowDown) => {
                self.camera.rotate(0.0, 0.1);
            }
            Key::Named(NamedKey::ArrowLeft) => {
                self.camera.rotate(-0.1, 0.0);
            }
            Key::Named(NamedKey::ArrowRight) => {
                self.camera.rotate(0.1, 0.0);
            }
            Key::Character(ref c) if c == "+" || c == "=" => {
                self.camera.zoom(-0.5);
            }
            Key::Character(ref c) if c == "-" => {
                self.camera.zoom(0.5);
            }
            Key::Named(NamedKey::Escape) => {
                self.menu.toggle();
            }
            Key::Character(ref c) if c == "m" || c == "M" => {
                self.menu.toggle();
            }
            Key::Character(ref c) if c == "i" || c == "I" => {
                if self.image_viewer.has_image() {
                    self.image_viewer.toggle_mode();
                }
            }
            Key::Character(ref c) if c == "h" || c == "H" => {
                self.show_info = !self.show_info;
            }
            Key::Character(ref c) if c == "s" || c == "S" => {
                if let Err(e) = self.config.save() {
                    eprintln!("Failed to save config: {}", e);
                }
            }
            _ => {}
        }
    }

    fn load_file(&mut self, renderer: &mut Renderer, path: &str) {
        if let Some(ext) = std::path::Path::new(path).extension().and_then(|e| e.to_str()) {
            match ext.to_lowercase().as_str() {
                "obj" => {
                    match Model::from_obj(path) {
                        Ok(mut model) => {
                            model.calculate_normals();
                            self.model_info = Some(ModelInfo::from_model(&model, Some(path.to_string())));
                            self.model = Some(model);
                            self.config.add_recent_file(path.to_string());
                            if let Err(e) = self.config.save() {
                                eprintln!("Failed to save config: {}", e);
                            }
                            println!("Loaded OBJ model: {}", path);
                        }
                        Err(e) => {
                            eprintln!("Failed to load OBJ: {}", e);
                        }
                    }
                }
                "png" | "jpg" | "jpeg" | "bmp" | "gif" | "webp" => {
                    if let Err(e) = self.image_viewer.load_image(
                        renderer.device(),
                        renderer.queue(),
                        path,
                    ) {
                        eprintln!("Failed to load image: {}", e);
                    } else {
                        self.config.add_recent_file(path.to_string());
                        if let Err(e) = self.config.save() {
                            eprintln!("Failed to save config: {}", e);
                        }
                        println!("Loaded image: {}", path);
                    }
                }
                _ => {
                    eprintln!("Unsupported file type: {:?}", ext);
                }
            }
        }
    }
}

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window = Rc::new(WindowBuilder::new()
        .with_title("Bookish 3D Viewer")
        .with_inner_size(winit::dpi::LogicalSize::new(800, 600))
        .build(&event_loop)
        .unwrap());

    let mut app = App::new();

    let mut renderer = match pollster::block_on(Renderer::new(window.as_ref())) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Failed to initialize renderer: {}", e);
            eprintln!("\nTroubleshooting:");
            eprintln!("1. Make sure you have updated graphics drivers");
            eprintln!("2. On Linux, ensure Vulkan drivers are installed:");
            eprintln!("   - For NVIDIA: nvidia-vulkan-driver");
            eprintln!("   - For AMD: mesa-vulkan-drivers");
            eprintln!("   - For Intel: vulkan-intel");
            eprintln!("3. Try running with: RUST_LOG=warn cargo run");
            std::process::exit(1);
        }
    };

    if let Some(path) = std::env::args().nth(1) {
        app.load_file(&mut renderer, &path);
    }

    let window_clone = window.clone();
    event_loop.run(move |event, elwt| {
        elwt.set_control_flow(ControlFlow::Poll);

        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window_clone.id() => match event {
                WindowEvent::CloseRequested => elwt.exit(),
                WindowEvent::Resized(physical_size) => {
                    renderer.resize(physical_size.width, physical_size.height);
                    app.camera.update_aspect(physical_size.width as f32, physical_size.height as f32);
                }
                WindowEvent::ScaleFactorChanged { .. } => {
                    let new_size = window_clone.inner_size();
                    renderer.resize(new_size.width, new_size.height);
                    app.camera.update_aspect(new_size.width as f32, new_size.height as f32);
                }
                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            state: ElementState::Pressed,
                            logical_key: key,
                            ..
                        },
                    ..
                } => {
                    match key {
                        Key::Character(ref c) if c == "w" || c == "W" => renderer.toggle_wireframe(),
                        Key::Character(ref c) if c == "f" || c == "F" => renderer.toggle_flat_shading(),
                        _ => app.handle_keyboard(key),
                    }
                }
                WindowEvent::MouseInput {
                    button: MouseButton::Left,
                    state: ElementState::Pressed,
                    ..
                } => {
                    app.mouse_pressed = true;
                }
                WindowEvent::MouseInput {
                    button: MouseButton::Left,
                    state: ElementState::Released,
                    ..
                } => {
                    app.mouse_pressed = false;
                }
                WindowEvent::MouseInput {
                    button: MouseButton::Right,
                    state: ElementState::Pressed,
                    ..
                } => {
                    app.right_mouse_pressed = true;
                }
                WindowEvent::MouseInput {
                    button: MouseButton::Right,
                    state: ElementState::Released,
                    ..
                } => {
                    app.right_mouse_pressed = false;
                }
                WindowEvent::CursorMoved { position, .. } => {
                    if app.mouse_pressed {
                        let delta_x = position.x - app.last_mouse_pos.0;
                        let delta_y = position.y - app.last_mouse_pos.1;
                        app.camera.rotate(delta_x as f32 * app.config.camera_sensitivity, delta_y as f32 * app.config.camera_sensitivity);
                    }
                    if app.right_mouse_pressed {
                        let delta_x = position.x - app.last_mouse_pos.0;
                        let delta_y = position.y - app.last_mouse_pos.1;
                        app.camera.pan(delta_x as f32 * app.config.pan_sensitivity, delta_y as f32 * app.config.pan_sensitivity);
                    }
                    app.last_mouse_pos = (position.x, position.y);
                }
                WindowEvent::MouseWheel { delta, .. } => {
                    match delta {
                        MouseScrollDelta::LineDelta(_, y) => {
                            app.camera.zoom(-y * app.config.zoom_sensitivity);
                        }
                        MouseScrollDelta::PixelDelta(pos) => {
                            app.camera.zoom(-(pos.y as f32) * app.config.zoom_sensitivity * 0.01);
                        }
                    }
                }
                WindowEvent::DroppedFile(path) => {
                    app.load_file(&mut renderer, path.to_str().unwrap());
                }
                _ => {}
            },
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                window_id,
            } if window_id == window_clone.id() => {
                let image_plane = if app.image_viewer.has_image()
                    && app.image_viewer.mode == image_viewer::ImageMode::Texture3D
                {
                    if let (Some(vb), Some(ib), Some(bg)) = (
                        app.image_viewer.vertex_buffer.as_ref(),
                        app.image_viewer.index_buffer.as_ref(),
                        app.image_viewer.bind_group.as_ref(),
                    ) {
                        Some((vb, ib, app.image_viewer.index_count, bg))
                    } else {
                        None
                    }
                } else {
                    None
                };

                match renderer.render(
                    &app.camera,
                    app.model.as_ref(),
                    image_plane,
                    app.config.wireframe_color,
                    app.config.flat_color,
                ) {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => renderer.resize(renderer.size().0, renderer.size().1),
                    Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                    Err(e) => eprintln!("Render error: {:?}", e),
                }
            }
            Event::AboutToWait => {
                window_clone.request_redraw();
            }
            _ => {}
        }
    }).unwrap();
}

