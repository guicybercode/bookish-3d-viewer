defmodule Bookish3dViewer.Application do
  @moduledoc false

  use Application

  @impl true
  def start(_type, _args) do
    children = [
      Bookish3dViewer.Supervisor
    ]

    opts = [strategy: :one_for_one, name: Bookish3dViewer.Application]
    Supervisor.start_link(children, opts)
  end
end

