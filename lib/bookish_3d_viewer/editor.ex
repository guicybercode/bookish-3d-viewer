defmodule Bookish3dViewer.Editor do
  @moduledoc """
  Editor operations and state management.
  """

  use GenServer

  defstruct [
    :transform,
    :selection,
    :color_picker,
    :history,
    :history_index,
    :max_history
  ]

  def start_link(_opts) do
    GenServer.start_link(__MODULE__, %{}, name: __MODULE__)
  end

  def translate(delta_x, delta_y, delta_z) do
    GenServer.cast(__MODULE__, {:translate, delta_x, delta_y, delta_z})
  end

  def rotate(delta_x, delta_y, delta_z) do
    GenServer.cast(__MODULE__, {:rotate, delta_x, delta_y, delta_z})
  end

  def scale(factor_x, factor_y, factor_z) do
    GenServer.cast(__MODULE__, {:scale, factor_x, factor_y, factor_z})
  end

  def undo do
    GenServer.call(__MODULE__, :undo)
  end

  def redo do
    GenServer.call(__MODULE__, :redo)
  end

  def reset do
    GenServer.cast(__MODULE__, :reset)
  end

  def set_wireframe_color(color) do
    GenServer.cast(__MODULE__, {:set_wireframe_color, color})
  end

  def set_flat_color(color) do
    GenServer.cast(__MODULE__, {:set_flat_color, color})
  end

  def set_background_color(color) do
    GenServer.cast(__MODULE__, {:set_background_color, color})
  end

  @impl true
  def init(_opts) do
    state = %__MODULE__{
      transform: %{translation: {0.0, 0.0, 0.0}, rotation: {0.0, 0.0, 0.0}, scale: {1.0, 1.0, 1.0}},
      selection: %{selected_vertices: [], selected_faces: [], mode: :none},
      color_picker: %{wireframe_color: 0x00FF00, flat_color: 0xFFBF00, background_color: 0x000000},
      history: [%{transform: %{translation: {0.0, 0.0, 0.0}, rotation: {0.0, 0.0, 0.0}, scale: {1.0, 1.0, 1.0}}}],
      history_index: 0,
      max_history: 100
    }
    {:ok, state}
  end

  @impl true
  def handle_call(:undo, _from, state) do
    if state.history_index > 0 do
      new_index = state.history_index - 1
      new_transform = Enum.at(state.history, new_index).transform
      new_state = %{state | history_index: new_index, transform: new_transform}
      {:reply, {:ok, new_transform}, new_state}
    else
      {:reply, {:error, "Nothing to undo"}, state}
    end
  end

  @impl true
  def handle_call(:redo, _from, state) do
    if state.history_index < length(state.history) - 1 do
      new_index = state.history_index + 1
      new_transform = Enum.at(state.history, new_index).transform
      new_state = %{state | history_index: new_index, transform: new_transform}
      {:reply, {:ok, new_transform}, new_state}
    else
      {:reply, {:error, "Nothing to redo"}, state}
    end
  end

  @impl true
  def handle_cast({:translate, delta_x, delta_y, delta_z}, state) do
    {tx, ty, tz} = state.transform.translation
    new_transform = %{state.transform | translation: {tx + delta_x, ty + delta_y, tz + delta_z}}
    new_state = save_state(%{state | transform: new_transform})
    {:noreply, new_state}
  end

  @impl true
  def handle_cast({:rotate, delta_x, delta_y, delta_z}, state) do
    {rx, ry, rz} = state.transform.rotation
    new_transform = %{state.transform | rotation: {rx + delta_x, ry + delta_y, rz + delta_z}}
    new_state = save_state(%{state | transform: new_transform})
    {:noreply, new_state}
  end

  @impl true
  def handle_cast({:scale, factor_x, factor_y, factor_z}, state) do
    {sx, sy, sz} = state.transform.scale
    new_transform = %{state.transform | scale: {sx * factor_x, sy * factor_y, sz * factor_z}}
    new_state = save_state(%{state | transform: new_transform})
    {:noreply, new_state}
  end

  @impl true
  def handle_cast(:reset, state) do
    default_transform = %{translation: {0.0, 0.0, 0.0}, rotation: {0.0, 0.0, 0.0}, scale: {1.0, 1.0, 1.0}}
    new_state = save_state(%{state | transform: default_transform})
    {:noreply, new_state}
  end

  @impl true
  def handle_cast({:set_wireframe_color, color}, state) do
    new_color_picker = %{state.color_picker | wireframe_color: color}
    {:noreply, %{state | color_picker: new_color_picker}}
  end

  @impl true
  def handle_cast({:set_flat_color, color}, state) do
    new_color_picker = %{state.color_picker | flat_color: color}
    {:noreply, %{state | color_picker: new_color_picker}}
  end

  @impl true
  def handle_cast({:set_background_color, color}, state) do
    new_color_picker = %{state.color_picker | background_color: color}
    {:noreply, %{state | color_picker: new_color_picker}}
  end

  defp save_state(state) do
    new_history = Enum.slice(state.history, 0..state.history_index) ++ [%{transform: state.transform}]
    truncated_history = if length(new_history) > state.max_history do
      Enum.slice(new_history, -state.max_history..-1)
    else
      new_history
    end
    %{state | history: truncated_history, history_index: length(truncated_history) - 1}
  end
end

