defmodule Bookish3dViewer.Supervisor do
  @moduledoc """
  Supervisor for Bookish3dViewer processes.
  """

  use Supervisor

  def start_link(init_arg) do
    Supervisor.start_link(__MODULE__, init_arg, name: __MODULE__)
  end

  @impl true
  def init(_init_arg) do
    children = [
      {Bookish3dViewer.Viewer, []},
      {Bookish3dViewer.Editor, []}
    ]

    Supervisor.init(children, strategy: :one_for_one)
  end
end

