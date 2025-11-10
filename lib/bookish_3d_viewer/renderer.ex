defmodule Bookish3dViewer.Renderer do
  @moduledoc """
  Rendering coordination and state management.
  """

  defstruct [
    :wireframe_mode,
    :flat_shading,
    :wireframe_color,
    :flat_color,
    :background_color
  ]

  @type t :: %__MODULE__{
    wireframe_mode: boolean(),
    flat_shading: boolean(),
    wireframe_color: integer(),
    flat_color: integer(),
    background_color: integer()
  }

  def new do
    %__MODULE__{
      wireframe_mode: false,
      flat_shading: true,
      wireframe_color: 0x00FF00,
      flat_color: 0xFFBF00,
      background_color: 0x000000
    }
  end

  def toggle_wireframe(renderer) do
    %{renderer | wireframe_mode: !renderer.wireframe_mode}
  end

  def toggle_flat_shading(renderer) do
    %{renderer | flat_shading: !renderer.flat_shading}
  end

  def set_wireframe_color(renderer, color) do
    %{renderer | wireframe_color: color}
  end

  def set_flat_color(renderer, color) do
    %{renderer | flat_color: color}
  end

  def set_background_color(renderer, color) do
    %{renderer | background_color: color}
  end
end

