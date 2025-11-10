defmodule Bookish3dViewer.Viewer do
  @moduledoc """
  GenServer for managing the 3D viewer state.
  """

  use GenServer

  def start_link(_opts) do
    GenServer.start_link(__MODULE__, %{}, name: __MODULE__)
  end

  def load_model(path) do
    GenServer.call(__MODULE__, {:load_model, path})
  end

  def rotate_camera(delta_x, delta_y) do
    GenServer.cast(__MODULE__, {:rotate_camera, delta_x, delta_y})
  end

  def zoom_camera(delta) do
    GenServer.cast(__MODULE__, {:zoom_camera, delta})
  end

  def pan_camera(delta_x, delta_y) do
    GenServer.cast(__MODULE__, {:pan_camera, delta_x, delta_y})
  end

  def reset_camera do
    GenServer.cast(__MODULE__, :reset_camera)
  end

  def get_model_info do
    GenServer.call(__MODULE__, :get_model_info)
  end

  @impl true
  def init(_opts) do
    {:ok, %{resource: nil}}
  end

  @impl true
  def handle_call({:load_model, path}, _from, state) do
    case Bookish3dViewer.Native.load_model(path) do
      {:ok, resource} ->
        {:reply, :ok, %{state | resource: resource}}
      error ->
        {:reply, error, state}
    end
  end

  @impl true
  def handle_call(:get_model_info, _from, state) do
    if state.resource do
      info = Bookish3dViewer.Native.get_model_info(state.resource)
      {:reply, info, state}
    else
      {:reply, {:error, "No model loaded"}, state}
    end
  end

  @impl true
  def handle_cast({:rotate_camera, delta_x, delta_y}, state) do
    if state.resource do
      Bookish3dViewer.Native.rotate_camera(state.resource, delta_x, delta_y)
    end
    {:noreply, state}
  end

  @impl true
  def handle_cast({:zoom_camera, delta}, state) do
    if state.resource do
      Bookish3dViewer.Native.zoom_camera(state.resource, delta)
    end
    {:noreply, state}
  end

  @impl true
  def handle_cast({:pan_camera, delta_x, delta_y}, state) do
    if state.resource do
      Bookish3dViewer.Native.pan_camera(state.resource, delta_x, delta_y)
    end
    {:noreply, state}
  end

  @impl true
  def handle_cast(:reset_camera, state) do
    if state.resource do
      Bookish3dViewer.Native.reset_camera(state.resource)
    end
    {:noreply, state}
  end
end

