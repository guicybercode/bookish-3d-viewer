defmodule Bookish3dViewer.Native do
  @moduledoc """
  NIF bindings to Rust code.
  """

  use Rustler, otp_app: :bookish_3d_viewer, crate: :bookish_nif

  def load_model(_path), do: :erlang.nif_error(:nif_not_loaded)
  def rotate_camera(_resource, _delta_x, _delta_y), do: :erlang.nif_error(:nif_not_loaded)
  def zoom_camera(_resource, _delta), do: :erlang.nif_error(:nif_not_loaded)
  def pan_camera(_resource, _delta_x, _delta_y), do: :erlang.nif_error(:nif_not_loaded)
  def reset_camera(_resource), do: :erlang.nif_error(:nif_not_loaded)
  def get_model_info(_resource), do: :erlang.nif_error(:nif_not_loaded)
end

