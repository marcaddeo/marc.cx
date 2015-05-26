defmodule MarcCx.LayoutView do
  use MarcCx.Web, :view

  def navigation(conn) do
    nav = [
      {:index, %{:text => "Home", :href => page_path(conn, :index), :active => false}},
      #{:about, %{:text => "About", :href => page_path(conn, :about), :active => false}},
      {:github, %{:text => "Github", :href => "https://github.com/marcaddeo", :active => false}},
    ]

    active = case conn.path_info do
      []              -> :index
      ["about"]       -> :about
      ["article" | _] -> :article
      _               -> :unknown
    end

    if Dict.has_key?(nav, active) do
      nav = Dict.update!(nav, active, fn (item) -> %{item | :active => true} end)
    end

    Dict.values(nav)
  end
end
