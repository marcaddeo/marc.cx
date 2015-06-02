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

  defp format_dates(articles) do
    Enum.map(articles, fn (article) ->
      format_date(article)
    end)
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

    timestamp = Date.convert(date, :secs)

    Map.put(%{metadata | :published => "#{month} #{day}, #{year}"}, :timestamp, timestamp)
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

  defp get_article_files() do
    Application.app_dir(:marc_cx, "priv/articles")
    |> File.ls!
  end

  defp get_articles() do
    get_article_files
    |> read_articles
    |> extract_metadata
    |> format_dates
    |> published
    |> sort_by_date :desc
  end

  defp extract_metadata(articles) do
    Enum.map(articles, fn (article) ->
      [slug | content] = article

      metadata = Markdown.get_metadata(content)

      Map.put(metadata, :slug, slug)
    end)
  end

  defp sort_by_date(articles, :desc) do
    Enum.sort(articles, fn (a, b) ->
      a.timestamp > b.timestamp
    end)
  end

  defp published(articles) do
    Enum.filter(articles, fn (article) ->
      article[:status] === "published"
    end)
  end

  defp sort_by_date(articles, :asc) do
    Enum.sort(articles, fn (a, b) ->
      a.timestamp < b.timestamp
    end)
  end
end
