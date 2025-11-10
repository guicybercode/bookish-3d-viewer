# Bookish 3D Viewer

A retro CAD-style 3D viewer application built in Rust, featuring an authentic 80s/90s aesthetic with wireframe and flat shading modes. This application supports viewing OBJ 3D models and 2D images (PNG, JPG) with a nostalgic green/amber color palette reminiscent of early CAD systems.

## Features

- **3D Model Viewing**: Load and display OBJ format 3D models with smooth rendering
- **Image Support**: View images as textures on 3D planes or in 2D viewer mode
- **Retro Visual Style**: Authentic 80s/90s CAD aesthetic with:
  - Green wireframe mode (#00FF00)
  - Amber flat shading mode (#FFBF00)
  - Black background for that classic terminal look
- **Interactive Camera Controls**: 
  - Mouse-based rotation, panning, and zooming
  - Keyboard shortcuts for precise control
- **Multiple Rendering Modes**: Toggle between wireframe and flat shading
- **Simple Menu System**: Access credits, controls, and information

## Installation

### Prerequisites

- Rust 1.70 or later (install from [rustup.rs](https://rustup.rs/))
- A graphics card with Vulkan, Metal, DirectX 12, or WebGPU support

### Building from Source

```bash
git clone https://github.com/guicybercode/bookish-3d-viewer.git
cd bookish-3d-viewer
cargo build --release
```

The executable will be located at `target/release/bookish-3d-viewer` (or `target/release/bookish-3d-viewer.exe` on Windows).

### Running

```bash
cargo run --release
```

Or with a file:

```bash
cargo run --release -- path/to/model.obj
cargo run --release -- path/to/image.png
```

## Usage

### Loading Files

- **Command Line**: Pass the file path as an argument when launching the application
- **Drag and Drop**: Drag OBJ files or images (PNG, JPG) directly into the window

### Controls

#### Mouse Controls

- **Left Click + Drag**: Rotate the camera around the model
- **Right Click + Drag**: Pan the view
- **Scroll Wheel**: Zoom in/out

#### Keyboard Shortcuts

- **R**: Reset camera to default position
- **W**: Toggle wireframe rendering mode
- **F**: Toggle flat shading mode
- **Arrow Keys**: Rotate view (Up/Down/Left/Right)
- **+ / =**: Zoom in
- **-**: Zoom out
- **M / ESC**: Toggle menu visibility
- **I**: Toggle image mode (when an image is loaded)

## Technical Details

### Architecture

The application is built using modern Rust graphics libraries:

- **wgpu**: Cross-platform graphics API abstraction (Vulkan/Metal/DX12/WebGPU)
- **winit**: Window management and event handling
- **glam**: Fast math library for 3D transformations
- **obj-rs**: OBJ file format parser
- **image**: Image loading and processing

### Project Structure

```
bookish-3d-viewer/
├── Cargo.toml          # Project dependencies and metadata
├── README.md           # This file
└── src/
    ├── main.rs         # Application entry point and event loop
    ├── renderer.rs     # WGPU rendering pipeline and shader management
    ├── camera.rs        # Arcball camera implementation
    ├── model.rs         # OBJ model loading and vertex processing
    ├── image_viewer.rs  # Image loading and texture management
    ├── menu.rs          # Menu system and UI overlay
    ├── utils.rs         # Utility functions for math and color conversion
    └── shaders/
        ├── wireframe.wgsl  # Wireframe rendering shader
        ├── flat.wgsl       # Flat shading shader
        └── image.wgsl      # Image texture shader
```

### Rendering Pipeline

The application uses three distinct rendering pipelines:

1. **Wireframe Pipeline**: Renders models as line primitives with green color
2. **Flat Shading Pipeline**: Renders models with flat faces and simple lighting using amber tones
3. **Image Pipeline**: Renders images as textures on 3D planes with proper UV mapping

### Camera System

The camera implements an arcball/orbit control system:
- Spherical coordinate system for smooth rotation
- Perspective projection with configurable FOV
- Distance-based zooming
- Pan support for precise positioning

## Supported File Formats

- **3D Models**: OBJ (.obj)
- **Images**: PNG (.png), JPEG (.jpg, .jpeg)

## Performance

The application is optimized for smooth 60 FPS rendering:
- Efficient vertex buffer management
- Minimal draw calls
- Hardware-accelerated rendering via wgpu
- Proper depth testing and culling

## Platform Support

- **Windows**: DirectX 12 or Vulkan
- **macOS**: Metal
- **Linux**: Vulkan
- **Web**: WebGPU (future support)

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## License

This project is licensed under the MIT License.

## Credits

**Made by**: [guicybercode](https://github.com/guicybercode)

**Repository**: [https://github.com/guicybercode/bookish-3d-viewer](https://github.com/guicybercode/bookish-3d-viewer)

### Acknowledgments

Built with love for retro computing aesthetics and modern Rust performance.

---

**하나님은 나의 빛이요, 나의 구원이시니 내가 누구를 두려워하랴**  
*God is my light and my salvation—whom shall I fear.*  
*(Psalm 27:1)*

# bookish-3d-viewer
