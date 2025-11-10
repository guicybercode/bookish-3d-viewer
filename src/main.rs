use bookish_3d_viewer::{App, renderer::Renderer, image_viewer::ImageMode};
use std::rc::Rc;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    keyboard::{Key, NamedKey},
};

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
                        _ => {
                            let key_str = match key {
                                Key::Character(c) => c.as_str(),
                                Key::Named(NamedKey::ArrowUp) => "ArrowUp",
                                Key::Named(NamedKey::ArrowDown) => "ArrowDown",
                                Key::Named(NamedKey::ArrowLeft) => "ArrowLeft",
                                Key::Named(NamedKey::ArrowRight) => "ArrowRight",
                                Key::Named(NamedKey::Escape) => "Escape",
                                _ => "",
                            };
                            app.handle_keyboard(key_str);
                        }
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
                    && app.image_viewer.mode == ImageMode::Texture3D
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

