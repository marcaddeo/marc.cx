defmodule MarcCx.PageController do
  use MarcCx.Web, :controller
  alias MarcCx.Article

  plug :action

  def index(conn, _params) do
    articles = Article.published |> Article.sort_by(:timestamp, :desc)

    render conn, "index.html", articles: articles
  end

  def about(conn, _params) do
    render conn, "about.html"
  end
end
