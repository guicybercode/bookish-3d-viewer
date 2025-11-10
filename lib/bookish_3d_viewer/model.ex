defmodule Bookish3dViewer.Model do
  @moduledoc """
  Model data structures and operations.
  """

  defstruct [
    :vertex_count,
    :face_count,
    :bounds_min,
    :bounds_max,
    :center,
    :file_path,
    :file_size
  ]

  @type t :: %__MODULE__{
    vertex_count: integer(),
    face_count: integer(),
    bounds_min: {float(), float(), float()},
    bounds_max: {float(), float(), float()},
    center: {float(), float(), float()},
    file_path: String.t() | nil,
    file_size: integer() | nil
  }

  def new(info_string) when is_binary(info_string) do
    parse_info_string(info_string)
  end

  defp parse_info_string(info) do
    lines = String.split(info, "\n")
    file_info = Enum.at(lines, 0)
    vertex_count = extract_integer(lines, "Vertices:")
    face_count = extract_integer(lines, "Faces:")
    center = extract_vector(lines, "Center:")
    bounds_min = extract_vector(lines, "Bounds:")
    bounds_max = extract_vector_second(lines, "Bounds:")

    %__MODULE__{
      vertex_count: vertex_count,
      face_count: face_count,
      bounds_min: bounds_min,
      bounds_max: bounds_max,
      center: center,
      file_path: extract_file_path(file_info),
      file_size: nil
    }
  end

  defp extract_integer(lines, prefix) do
    case Enum.find(lines, fn line -> String.starts_with?(line, prefix) end) do
      nil -> 0
      line ->
        line
        |> String.replace(prefix, "")
        |> String.trim()
        |> String.to_integer()
    end
  end

  defp extract_vector(lines, prefix) do
    case Enum.find(lines, fn line -> String.starts_with?(line, prefix) end) do
      nil -> {0.0, 0.0, 0.0}
      line ->
        coords = line
        |> String.replace(prefix, "")
        |> String.replace("(", "")
        |> String.replace(")", "")
        |> String.split(",")
        |> Enum.map(&String.trim/1)
        |> Enum.map(&String.to_float/1)

        case coords do
          [x, y, z] -> {x, y, z}
          _ -> {0.0, 0.0, 0.0}
        end
    end
  end

  defp extract_vector_second(lines, prefix) do
    case Enum.find(lines, fn line -> String.starts_with?(line, prefix) end) do
      nil -> {0.0, 0.0, 0.0}
      line ->
        parts = String.split(line, "to")
        if length(parts) == 2 do
          coords = parts
          |> Enum.at(1)
          |> String.replace("(", "")
          |> String.replace(")", "")
          |> String.split(",")
          |> Enum.map(&String.trim/1)
          |> Enum.map(&String.to_float/1)

          case coords do
            [x, y, z] -> {x, y, z}
            _ -> {0.0, 0.0, 0.0}
          end
        else
          {0.0, 0.0, 0.0}
        end
    end
  end

  defp extract_file_path(file_info) do
    case String.split(file_info, "File:") do
      [_prefix, path] -> String.trim(path)
      _ -> nil
    end
  end
end

