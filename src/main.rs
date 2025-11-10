mod camera;
mod model;
mod renderer;
mod image_viewer;
mod menu;
mod utils;

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

struct App {
    camera: Camera,
    model: Option<Model>,
    image_viewer: ImageViewer,
    menu: Menu,
    mouse_pressed: bool,
    right_mouse_pressed: bool,
    last_mouse_pos: (f64, f64),
}

impl App {
    fn new() -> Self {
        Self {
            camera: Camera::new(800.0, 600.0),
            model: None,
            image_viewer: ImageViewer::new(),
            menu: Menu::new(),
            mouse_pressed: false,
            right_mouse_pressed: false,
            last_mouse_pos: (0.0, 0.0),
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
            _ => {}
        }
    }

    fn load_file(&mut self, renderer: &mut Renderer, path: &str) {
        if let Ok(ext) = std::path::Path::new(path).extension() {
            match ext.to_str().unwrap().to_lowercase().as_str() {
                "obj" => {
                    match Model::from_obj(path) {
                        Ok(mut model) => {
                            model.calculate_normals();
                            self.model = Some(model);
                            println!("Loaded OBJ model: {}", path);
                        }
                        Err(e) => {
                            eprintln!("Failed to load OBJ: {}", e);
                        }
                    }
                }
                "png" | "jpg" | "jpeg" => {
                    if let Err(e) = self.image_viewer.load_image(
                        renderer.device(),
                        renderer.queue(),
                        path,
                    ) {
                        eprintln!("Failed to load image: {}", e);
                    } else {
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
    let window = WindowBuilder::new()
        .with_title("Bookish 3D Viewer")
        .with_inner_size(winit::dpi::LogicalSize::new(800, 600))
        .build(&event_loop)
        .unwrap();

    let mut app = App::new();

    let mut renderer = pollster::block_on(Renderer::new(&window)).unwrap();

    if let Some(path) = std::env::args().nth(1) {
        app.load_file(&mut renderer, &path);
    }

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(physical_size) => {
                    renderer.resize(physical_size.width, physical_size.height);
                    app.camera.update_aspect(physical_size.width as f32, physical_size.height as f32);
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    renderer.resize(new_inner_size.width, new_inner_size.height);
                    app.camera.update_aspect(new_inner_size.width as f32, new_inner_size.height as f32);
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
                        app.camera.rotate(delta_x as f32, delta_y as f32);
                    }
                    if app.right_mouse_pressed {
                        let delta_x = position.x - app.last_mouse_pos.0;
                        let delta_y = position.y - app.last_mouse_pos.1;
                        app.camera.pan(delta_x as f32, delta_y as f32);
                    }
                    app.last_mouse_pos = (position.x, position.y);
                }
                WindowEvent::MouseWheel { delta, .. } => {
                    match delta {
                        MouseScrollDelta::LineDelta(_, y) => {
                            app.camera.zoom(-y * 0.1);
                        }
                        MouseScrollDelta::PixelDelta(pos) => {
                            app.camera.zoom(-(pos.y as f32) * 0.001);
                        }
                    }
                }
                WindowEvent::DroppedFile(path) => {
                    app.load_file(&mut renderer, path.to_str().unwrap());
                }
                _ => {}
            },
            Event::RedrawRequested(_) => {
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

                match renderer.render(&app.camera, app.model.as_ref(), image_plane) {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => renderer.resize(renderer.size().0, renderer.size().1),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => eprintln!("Render error: {:?}", e),
                }
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }
    }).unwrap();
}

