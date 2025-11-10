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

    pub fn render_text(&self) -> String {
        if !self.visible {
            return String::new();
        }

        format!(
            r#"
╔═══════════════════════════════════════════════════════════╗
║                  BOOKISH 3D VIEWER                       ║
║                                                           ║
║  CREDITS                                                  ║
║  ─────────────────────────────────────────────────────  ║
║  Made by: guicybercode                                    ║
║  Repository: https://github.com/guicybercode/bookish-3d-viewer ║
║                                                           ║
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
║                                                           ║
║  FILE LOADING                                            ║
║  ─────────────────────────────────────────────────────  ║
║  Drag and drop OBJ files or images (PNG, JPG)            ║
║  into the window, or pass file path as argument          ║
║                                                           ║
║  VERSION                                                 ║
║  ─────────────────────────────────────────────────────  ║
║  Version 0.1.0                                           ║
║                                                           ║
║  Press M or ESC to close this menu                       ║
╚═══════════════════════════════════════════════════════════╝
"#
        )
    }
}

