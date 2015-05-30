defmodule MarcCx.PageController do
  use MarcCx.Web, :controller
  use Timex
  alias MarcCx.Markdown

  plug :action

  def index(conn, _params) do
    render conn, "index.html", articles: get_articles()
  end

  def about(conn, _params) do
    render conn, "about.html"
  end

  defp format_date(%{:published => published} = metadata) do
    {:ok, date} = DateFormat.parse(published, "{M}/{D}/{YYYY}")

    [month, day, year] = date
    |> DateFormat.format!("%B %d %Y", :strftime)
    |> String.split(" ")

    day = day
    |> String.lstrip(?0)

    day = case String.last(day) do
      "1" -> "#{day}st"
      "2" -> "#{day}nd"
      "3" -> "#{day}rd"
      _   -> "#{day}th"
    end

    %{metadata | :published => "#{month} #{day}, #{year}"}
  end

  defp read_articles([], articles), do: articles
  defp read_articles([file | files], articles) do
    path = Application.app_dir(:marc_cx, "priv/articles/#{file}")
    content = File.read!(path)
    |> String.split(~r{\r\n?|\n})
    slug = Path.basename(file, ".md")

    read_articles(files, articles ++ [[slug | content]])
  end
  defp read_articles(files) when is_list(files) do
    read_articles(files, [])
  end

  defp get_articles() do
    path = Application.app_dir(:marc_cx, "priv/articles")
    {:ok, files} = File.ls(path)

    articles = read_articles(files)

    articles = files
    |> read_articles
    |> Enum.map(fn (article) ->
      [slug | content] = article

      metadata = content
      |> Markdown.get_metadata
      |> format_date

      Map.put(metadata, :slug, slug)
    end)

    articles
  end
end
