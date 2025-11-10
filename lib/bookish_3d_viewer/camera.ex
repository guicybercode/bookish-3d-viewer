defmodule Bookish3dViewer.Camera do
  @moduledoc """
  Camera state and operations.
  """

  defstruct [
    :position,
    :target,
    :fov,
    :aspect,
    :distance,
    :rotation_x,
    :rotation_y,
    :pan_x,
    :pan_y
  ]

  @type t :: %__MODULE__{
    position: {float(), float(), float()},
    target: {float(), float(), float()},
    fov: float(),
    aspect: float(),
    distance: float(),
    rotation_x: float(),
    rotation_y: float(),
    pan_x: float(),
    pan_y: float()
  }

  def new(width \\ 800.0, height \\ 600.0) do
    %__MODULE__{
      position: {0.0, 0.0, 5.0},
      target: {0.0, 0.0, 0.0},
      fov: 45.0,
      aspect: width / height,
      distance: 5.0,
      rotation_x: 0.0,
      rotation_y: 0.0,
      pan_x: 0.0,
      pan_y: 0.0
    }
  end

  def rotate(camera, delta_x, delta_y) do
    %{camera |
      rotation_x: camera.rotation_x + delta_x,
      rotation_y: camera.rotation_y + delta_y
    }
  end

  def zoom(camera, delta) do
    new_distance = max(0.1, camera.distance + delta)
    %{camera | distance: new_distance}
  end

  def pan(camera, delta_x, delta_y) do
    %{camera |
      pan_x: camera.pan_x + delta_x,
      pan_y: camera.pan_y + delta_y
    }
  end

  def reset(camera) do
    new(width: 800.0, height: 600.0)
  end
end

