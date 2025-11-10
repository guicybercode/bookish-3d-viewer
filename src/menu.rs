pub struct Menu {
    pub visible: bool,
}

impl Menu {
    pub fn new() -> Self {
        Self { visible: false }
    }

    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }

    pub fn render_text(&self, model_info: Option<&str>) -> String {
        if !self.visible {
            return String::new();
        }

        let model_section = if let Some(info) = model_info {
            format!("\n  MODEL INFO\n  ─────────────────────────────────────────────────────\n{}\n", info)
        } else {
            String::from("\n  MODEL INFO\n  ─────────────────────────────────────────────────────\n  No model loaded\n")
        };

        format!(
            r#"
╔═══════════════════════════════════════════════════════════╗
║                  BOOKISH 3D VIEWER                       ║
║                                                           ║
║  CREDITS                                                  ║
║  ─────────────────────────────────────────────────────  ║
║  Made by: guicybercode                                    ║
║  Repository: https://github.com/guicybercode/bookish-3d-viewer ║
║                                                           ║{}
║  CONTROLS                                                 ║
║  ─────────────────────────────────────────────────────  ║
║  Mouse:                                                   ║
║    Left Click + Drag    - Rotate view                    ║
║    Right Click + Drag   - Pan view                       ║
║    Scroll Wheel         - Zoom in/out                    ║
║                                                           ║
║  Keyboard:                                               ║
║    R                    - Reset camera                   ║
║    W                    - Toggle wireframe mode          ║
║    F                    - Toggle flat shading            ║
║    Arrow Keys           - Rotate view                    ║
║    +/-                  - Zoom in/out                    ║
║    M / ESC              - Toggle menu                    ║
║    I                    - Toggle image mode (if image)  ║
║    H                    - Toggle model info              ║
║    S                    - Save configuration             ║
║                                                           ║
║  FILE LOADING                                            ║
║  ─────────────────────────────────────────────────────  ║
║  Drag and drop OBJ files or images (PNG, JPG, BMP, etc) ║
║  into the window, or pass file path as argument          ║
║                                                           ║
║  VERSION                                                 ║
║  ─────────────────────────────────────────────────────  ║
║  Version 0.1.0                                           ║
║                                                           ║
║  Press M or ESC to close this menu                       ║
╚═══════════════════════════════════════════════════════════╝
"#,
            model_section
        )
    }
}

