defmodule Bookish3dViewer.Menu do
  @moduledoc """
  Menu system for displaying information and controls.
  """

  defstruct [:visible]

  def new do
    %__MODULE__{visible: false}
  end

  def toggle(menu) do
    %{menu | visible: !menu.visible}
  end

  def render_text(menu, model_info \\ nil) do
    if menu.visible do
      model_section = if model_info do
        "\n  MODEL INFO\n  ─────────────────────────────────────────────────────\n#{model_info}\n"
      else
        "\n  MODEL INFO\n  ─────────────────────────────────────────────────────\n  No model loaded\n"
      end

      """
╔═══════════════════════════════════════════════════════════╗
║                  BOOKISH 3D VIEWER                       ║
║                                                           ║
║  CREDITS                                                  ║
║  ─────────────────────────────────────────────────────  ║
║  Made by: guicybercode                                    ║
║  Repository: https://github.com/guicybercode/bookish-3d-viewer ║
║                                                           ║#{model_section}║  CONTROLS                                                 ║
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
"""
    else
      ""
    end
  end
end

