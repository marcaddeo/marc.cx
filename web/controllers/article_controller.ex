defmodule MarcCx.ArticleController do
  use MarcCx.Web, :controller
  alias MarcCx.Markdown

  plug :action

  def index(conn, %{"slug" => slug}) do
    case read_article(slug) do
      {:ok, content} ->
        {metadata, content} = Markdown.parse(content)

        assigns = %{
          content: content,
          title: metadata.title,
          published: metadata.published
        }

        render conn, "index.html", assigns
      {:error, :not_found} ->
        conn
        |> put_status(:not_found)
        |> render(MarcCx.ErrorView, "404.html")
    end
  end

  defp read_article(slug) do
    path = Application.app_dir(:marc_cx, "priv/articles/#{slug}.md")

    if File.exists?(path) do
      content = File.read!(path)
      |> String.split(~r{\r\n?|\n})

      {:ok, content}
    else
      {:error, :not_found}
    end
  end
end
