# Bookish 3D Viewer

A retro CAD-style 3D viewer application built in Rust, featuring an authentic 80s/90s aesthetic with wireframe and flat shading modes. This application supports viewing OBJ 3D models and 2D images (PNG, JPG, BMP, GIF, WebP) with a nostalgic green/amber color palette reminiscent of early CAD systems.

## Features

- **3D Model Viewing**: Load and display OBJ format 3D models with smooth rendering
- **Image Support**: View images as textures on 3D planes or in 2D viewer mode
  - Supported formats: PNG, JPG, JPEG, BMP, GIF, WebP
- **Retro Visual Style**: Authentic 80s/90s CAD aesthetic with:
  - Green wireframe mode (#00FF00)
  - Amber flat shading mode (#FFBF00)
  - Black background for that classic terminal look
  - Customizable colors via configuration
- **Interactive Camera Controls**: 
  - Mouse-based rotation, panning, and zooming
  - Keyboard shortcuts for precise control
  - Configurable sensitivity settings
- **Multiple Rendering Modes**: Toggle between wireframe and flat shading
- **Configuration System**: Persistent settings with TOML configuration file
- **Model Information**: Display detailed statistics about loaded models
- **Recent Files**: Track recently opened files
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
- **Drag and Drop**: Drag OBJ files or images (PNG, JPG, BMP, GIF, WebP) directly into the window

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
- **H**: Toggle model information display
- **S**: Save current configuration

## Configuration

The application automatically creates a configuration file at:
- **Linux**: `~/.config/bookish-3d-viewer/config.toml`
- **macOS**: `~/Library/Application Support/bookish-3d-viewer/config.toml`
- **Windows**: `%APPDATA%\bookish-3d-viewer\config.toml`

You can customize:
- Wireframe color
- Flat shading color
- Background color
- Camera sensitivity
- Zoom sensitivity
- Pan sensitivity
- Default field of view
- Recent files list

## Technical Details

### Architecture

The application is built using modern Rust graphics libraries:

- **wgpu**: Cross-platform graphics API abstraction (Vulkan/Metal/DX12/WebGPU)
- **winit**: Window management and event handling
- **glam**: Fast math library for 3D transformations
- **tobj**: OBJ file format parser
- **image**: Image loading and processing
- **serde/toml**: Configuration file management
- **dirs**: Cross-platform directory handling

### Project Structure

```
bookish-3d-viewer/
├── Cargo.toml          # Project dependencies and metadata
├── README.md           # This file
├── .gitignore          # Git ignore rules
└── src/
    ├── main.rs         # Application entry point and event loop
    ├── renderer.rs     # WGPU rendering pipeline and shader management
    ├── camera.rs        # Arcball camera implementation
    ├── model.rs         # OBJ model loading and vertex processing
    ├── model_info.rs    # Model statistics and information
    ├── image_viewer.rs  # Image loading and texture management
    ├── menu.rs          # Menu system and UI overlay
    ├── config.rs        # Configuration management
    ├── error.rs         # Error handling types
    ├── utils.rs         # Utility functions for math and color conversion
    └── shaders/
        ├── wireframe.wgsl  # Wireframe rendering shader
        ├── flat.wgsl       # Flat shading shader
        └── image.wgsl      # Image texture shader
```

### Rendering Pipeline

The application uses three distinct rendering pipelines:

1. **Wireframe Pipeline**: Renders models as line primitives with customizable color
2. **Flat Shading Pipeline**: Renders models with flat faces and simple lighting using customizable colors
3. **Image Pipeline**: Renders images as textures on 3D planes with proper UV mapping

### Camera System

The camera implements an arcball/orbit control system:
- Spherical coordinate system for smooth rotation
- Perspective projection with configurable FOV
- Distance-based zooming
- Pan support for precise positioning
- Configurable sensitivity for all controls

## Supported File Formats

- **3D Models**: OBJ (.obj)
- **Images**: PNG (.png), JPEG (.jpg, .jpeg), BMP (.bmp), GIF (.gif), WebP (.webp)

## Performance

The application is optimized for smooth 60 FPS rendering:
- Efficient vertex buffer management
- Minimal draw calls
- Hardware-accelerated rendering via wgpu
- Proper depth testing and culling
- Buffer reuse to minimize allocations

## Platform Support

- **Windows**: DirectX 12 or Vulkan
- **macOS**: Metal
- **Linux**: Vulkan
- **Web**: WebGPU (future support)

## Recent Improvements

- ✅ Configuration system with persistent settings
- ✅ Model information display
- ✅ Support for more image formats (BMP, GIF, WebP)
- ✅ Recent files tracking
- ✅ Configurable camera sensitivity
- ✅ Better error handling
- ✅ Improved code organization
- ✅ Enhanced menu with model info

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
