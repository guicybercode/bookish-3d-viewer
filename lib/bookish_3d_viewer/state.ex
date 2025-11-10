defmodule Bookish3dViewer.State do
  @moduledoc """
  Application state management.
  """

  defstruct [
    :viewer_resource,
    :camera,
    :renderer,
    :model,
    :config,
    :menu,
    :editor,
    :mouse_state,
    :keyboard_state
  ]

  @type t :: %__MODULE__{
    viewer_resource: term() | nil,
    camera: Bookish3dViewer.Camera.t() | nil,
    renderer: Bookish3dViewer.Renderer.t() | nil,
    model: Bookish3dViewer.Model.t() | nil,
    config: Bookish3dViewer.Config.t() | nil,
    menu: Bookish3dViewer.Menu.t() | nil,
    editor: term() | nil,
    mouse_state: map(),
    keyboard_state: map()
  }

  def new do
    %__MODULE__{
      viewer_resource: nil,
      camera: Bookish3dViewer.Camera.new(),
      renderer: Bookish3dViewer.Renderer.new(),
      model: nil,
      config: Bookish3dViewer.Config.load(),
      menu: Bookish3dViewer.Menu.new(),
      editor: nil,
      mouse_state: %{pressed: false, right_pressed: false, last_pos: {0.0, 0.0}},
      keyboard_state: %{}
    }
  end

  def update_camera(state, camera) do
    %{state | camera: camera}
  end

  def update_renderer(state, renderer) do
    %{state | renderer: renderer}
  end

  def update_model(state, model) do
    %{state | model: model}
  end

  def update_config(state, config) do
    %{state | config: config}
  end

  def update_menu(state, menu) do
    %{state | menu: menu}
  end

  def toggle_menu(state) do
    %{state | menu: Bookish3dViewer.Menu.toggle(state.menu)}
  end

  def update_mouse_state(state, mouse_state) do
    %{state | mouse_state: mouse_state}
  end

  def update_keyboard_state(state, keyboard_state) do
    %{state | keyboard_state: keyboard_state}
  end

  def set_viewer_resource(state, resource) do
    %{state | viewer_resource: resource}
  end

  def has_model?(state) do
    not is_nil(state.model)
  end

  def has_resource?(state) do
    not is_nil(state.viewer_resource)
  end
end

