defmodule MarcCx.ArticleController do
  use MarcCx.Web, :controller
  alias MarcCx.Article

  plug :action

  def index(conn, %{"slug" => slug}) do
    case Article.get_article(slug) do
      {:ok, article} ->
        assigns = %{
          content: article.html,
          title: article.title
        }

        render conn, "index.html", assigns
      {:error, :notfound} ->
        conn
        |> put_status(:not_found)
        |> render(MarcCx.ErrorView, "404.html")
    end
  end
end
