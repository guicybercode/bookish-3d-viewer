pub mod camera;
pub mod model;
pub mod image_viewer;
pub mod menu;
pub mod utils;
pub mod config;
pub mod model_info;
pub mod error;
pub mod transform;
pub mod color_picker;
pub mod selection;
pub mod editor;

#[cfg(target_arch = "wasm32")]
pub mod wasm;

#[cfg(feature = "desktop")]
use renderer::Renderer;

use camera::Camera;
use model::Model;
use image_viewer::ImageViewer;
use menu::Menu;
use config::Config;
use model_info::ModelInfo;

pub struct App {
    pub camera: Camera,
    pub model: Option<Model>,
    pub model_info: Option<ModelInfo>,
    pub image_viewer: ImageViewer,
    pub menu: Menu,
    pub config: Config,
    pub mouse_pressed: bool,
    pub right_mouse_pressed: bool,
    pub last_mouse_pos: (f64, f64),
    pub show_info: bool,
}

impl App {
    pub fn new() -> Self {
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

    pub fn handle_keyboard(&mut self, key: &str) {
        match key {
            "r" | "R" => {
                self.camera.reset();
            }
            "ArrowUp" => {
                self.camera.rotate(0.0, -0.1);
            }
            "ArrowDown" => {
                self.camera.rotate(0.0, 0.1);
            }
            "ArrowLeft" => {
                self.camera.rotate(-0.1, 0.0);
            }
            "ArrowRight" => {
                self.camera.rotate(0.1, 0.0);
            }
            "+" | "=" => {
                self.camera.zoom(-0.5);
            }
            "-" => {
                self.camera.zoom(0.5);
            }
            "Escape" => {
                self.menu.toggle();
            }
            "m" | "M" => {
                self.menu.toggle();
            }
            "i" | "I" => {
                if self.image_viewer.has_image() {
                    self.image_viewer.toggle_mode();
                }
            }
            "h" | "H" => {
                self.show_info = !self.show_info;
            }
            "s" | "S" => {
                if let Err(e) = self.config.save() {
                    eprintln!("Failed to save config: {}", e);
                }
            }
            _ => {}
        }
    }

    #[cfg(feature = "desktop")]
    pub fn load_file(&mut self, renderer: &mut Renderer, path: &str) {
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

    pub fn load_file_from_bytes(&mut self, data: &[u8], extension: &str) -> Result<(), Box<dyn std::error::Error>> {
        match extension.to_lowercase().as_str() {
            "obj" => {
                let mut model = Model::from_obj_bytes(data)?;
                model.calculate_normals();
                self.model_info = Some(ModelInfo::from_model(&model, None));
                self.model = Some(model);
                Ok(())
            }
            _ => Err(format!("Unsupported file type: {}", extension).into())
        }
    }
}

