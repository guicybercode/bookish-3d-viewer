defmodule Bookish3dViewer.FileHandler do
  @moduledoc """
  File handling and loading operations.
  """

  @supported_image_extensions ~w(png jpg jpeg bmp gif webp)
  @supported_model_extensions ~w(obj)

  def load_file(path) when is_binary(path) do
    if File.exists?(path) do
      ext = Path.extname(path) |> String.trim_leading(".") |> String.downcase()
      
      cond do
        ext in @supported_model_extensions ->
          load_model_file(path)
        ext in @supported_image_extensions ->
          load_image_file(path)
        true ->
          {:error, "Unsupported file type: #{ext}"}
      end
    else
      {:error, "File not found: #{path}"}
    end
  end

  defp load_model_file(path) do
    case Bookish3dViewer.Viewer.load_model(path) do
      :ok -> {:ok, :model, path}
      error -> error
    end
  end

  defp load_image_file(path) do
    case File.read(path) do
      {:ok, data} -> {:ok, :image, path, data}
      error -> error
    end
  end

  def is_supported?(path) when is_binary(path) do
    ext = Path.extname(path) |> String.trim_leading(".") |> String.downcase()
    ext in @supported_model_extensions or ext in @supported_image_extensions
  end

  def get_file_type(path) when is_binary(path) do
    ext = Path.extname(path) |> String.trim_leading(".") |> String.downcase()
    
    cond do
      ext in @supported_model_extensions -> :model
      ext in @supported_image_extensions -> :image
      true -> :unknown
    end
  end
end

