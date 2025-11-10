defmodule Bookish3dViewer.Error do
  @moduledoc """
  Error handling and error types.
  """

  defexception [:message]

  def new(message) when is_binary(message) do
    %__MODULE__{message: message}
  end

  def model_load_error(message) do
    new("Model load error: #{message}")
  end

  def image_load_error(message) do
    new("Image load error: #{message}")
  end

  def config_error(message) do
    new("Config error: #{message}")
  end

  def render_error(message) do
    new("Render error: #{message}")
  end
end

