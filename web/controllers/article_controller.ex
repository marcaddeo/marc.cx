defmodule MarcCx.ArticleController do
  use MarcCx.Web, :controller

  plug :action

  def index(conn, %{"slug" => slug}) do
    case read_article(slug) do
      {:ok, content} ->
        {metadata, content} = parse_metadata(content)

        content = Earmark.to_html(content, %Earmark.Options{
          renderer: MarcCx.HtmlRenderer,
          footnotes: true
        })

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

  defp parse_metadata(["__START_META__" | tail], metadata) do
    parse_metadata(tail, metadata)
  end
  defp parse_metadata(["__END_META__" | content], metadata) do
    {metadata, content}
  end
  defp parse_metadata([head | tail], metadata) do
    [key, value] = String.split(head, ":", parts: 2, trim: true)
    parse_metadata(tail, Dict.put(metadata, String.to_atom(key), String.strip(value)))
  end

  defp parse_metadata(content) when is_list(content) do
    parse_metadata(content, %{})
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
