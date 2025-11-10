defmodule Bookish3dViewer.Config do
  @moduledoc """
  Configuration management.
  """

  defstruct [
    :wireframe_color,
    :flat_color,
    :background_color,
    :camera_sensitivity,
    :zoom_sensitivity,
    :pan_sensitivity,
    :default_fov,
    :recent_files
  ]

  @type t :: %__MODULE__{
    wireframe_color: integer(),
    flat_color: integer(),
    background_color: integer(),
    camera_sensitivity: float(),
    zoom_sensitivity: float(),
    pan_sensitivity: float(),
    default_fov: float(),
    recent_files: list(String.t())
  }

  def load do
    config_path = config_path()
    if File.exists?(config_path) do
      case File.read(config_path) do
        {:ok, content} ->
          case Toml.decode(content) do
            {:ok, data} -> from_map(data)
            _ -> default()
          end
        _ -> default()
      end
    else
      default()
    end
  end

  def save(config) do
    config_path = config_path()
    config_dir = Path.dirname(config_path)
    File.mkdir_p!(config_dir)
    content = to_map(config) |> Toml.encode!()
    File.write!(config_path, content)
  end

  defp default do
    %__MODULE__{
      wireframe_color: 0x00FF00,
      flat_color: 0xFFBF00,
      background_color: 0x000000,
      camera_sensitivity: 0.01,
      zoom_sensitivity: 0.1,
      pan_sensitivity: 0.005,
      default_fov: 45.0,
      recent_files: []
    }
  end

  defp config_path do
    System.user_home!()
    |> Path.join(".config/bookish-3d-viewer/config.toml")
  end

  defp from_map(data) do
    %__MODULE__{
      wireframe_color: Map.get(data, "wireframe_color", 0x00FF00),
      flat_color: Map.get(data, "flat_color", 0xFFBF00),
      background_color: Map.get(data, "background_color", 0x000000),
      camera_sensitivity: Map.get(data, "camera_sensitivity", 0.01),
      zoom_sensitivity: Map.get(data, "zoom_sensitivity", 0.1),
      pan_sensitivity: Map.get(data, "pan_sensitivity", 0.005),
      default_fov: Map.get(data, "default_fov", 45.0),
      recent_files: Map.get(data, "recent_files", [])
    }
  end

  defp to_map(config) do
    %{
      "wireframe_color" => config.wireframe_color,
      "flat_color" => config.flat_color,
      "background_color" => config.background_color,
      "camera_sensitivity" => config.camera_sensitivity,
      "zoom_sensitivity" => config.zoom_sensitivity,
      "pan_sensitivity" => config.pan_sensitivity,
      "default_fov" => config.default_fov,
      "recent_files" => config.recent_files
    }
  end
end

