defmodule Bookish3dViewer.MixProject do
  use Mix.Project

  def project do
    [
      app: :bookish_3d_viewer,
      version: "0.1.0",
      elixir: "~> 1.14",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      compilers: [:rustler] ++ Mix.compilers(),
      rustler_crates: rustler_crates()
    ]
  end

  def application do
    [
      extra_applications: [:logger],
      mod: {Bookish3dViewer.Application, []}
    ]
  end

  defp deps do
    [
      {:rustler, "~> 0.29"},
      {:toml, "~> 0.6"}
    ]
  end

  defp rustler_crates do
    [
      bookish_nif: [
        path: "native/bookish_nif",
        mode: if(Mix.env() == :prod, do: :release, else: :debug)
      ]
    ]
  end
end

