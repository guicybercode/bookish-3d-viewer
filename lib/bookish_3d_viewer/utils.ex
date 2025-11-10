defmodule Bookish3dViewer.Utils do
  @moduledoc """
  Utility functions for the viewer.
  """

  def format_file_size(bytes) when is_integer(bytes) do
    cond do
      bytes < 1024 -> "#{bytes} B"
      bytes < 1024 * 1024 -> "#{:erlang.float_to_binary(bytes / 1024, decimals: 2)} KB"
      bytes < 1024 * 1024 * 1024 -> "#{:erlang.float_to_binary(bytes / (1024 * 1024), decimals: 2)} MB"
      true -> "#{:erlang.float_to_binary(bytes / (1024 * 1024 * 1024), decimals: 2)} GB"
    end
  end

  def get_file_name(path) when is_binary(path) do
    Path.basename(path)
  end

  def calculate_center(vertices) when is_list(vertices) do
    if Enum.empty?(vertices) do
      {0.0, 0.0, 0.0}
    else
      {sum_x, sum_y, sum_z, count} = Enum.reduce(vertices, {0.0, 0.0, 0.0, 0}, fn
        {x, y, z}, {acc_x, acc_y, acc_z, cnt} ->
          {acc_x + x, acc_y + y, acc_z + z, cnt + 1}
      end)
      {sum_x / count, sum_y / count, sum_z / count}
    end
  end

  def calculate_bounds(vertices) when is_list(vertices) do
    if Enum.empty?(vertices) do
      {{0.0, 0.0, 0.0}, {0.0, 0.0, 0.0}}
    else
      {min_x, min_y, min_z, max_x, max_y, max_z} = Enum.reduce(vertices, {nil, nil, nil, nil, nil, nil}, fn
        {x, y, z}, {min_x, min_y, min_z, max_x, max_y, max_z} ->
          {
            if is_nil(min_x), do: x, else: min(min_x, x),
            if is_nil(min_y), do: y, else: min(min_y, y),
            if is_nil(min_z), do: z, else: min(min_z, z),
            if is_nil(max_x), do: x, else: max(max_x, x),
            if is_nil(max_y), do: y, else: max(max_y, y),
            if is_nil(max_z), do: z, else: max(max_z, z)
          }
      end)
      {{min_x, min_y, min_z}, {max_x, max_y, max_z}}
    end
  end

  def color_to_rgba(color) when is_integer(color) do
    r = (color >>> 16) &&& 0xFF
    g = (color >>> 8) &&& 0xFF
    b = color &&& 0xFF
    {r / 255.0, g / 255.0, b / 255.0, 1.0}
  end

  def rgba_to_color({r, g, b, _a}) do
    (round(r * 255) <<< 16) ||| (round(g * 255) <<< 8) ||| round(b * 255)
  end
end

