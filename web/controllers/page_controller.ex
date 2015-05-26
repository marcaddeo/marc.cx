defmodule MarcCx.PageController do
  use MarcCx.Web, :controller
  use Timex

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

  defp parse_metadata(["__START_META__" | tail], metadata) do
    parse_metadata(tail, metadata)
  end
  defp parse_metadata(["__END_META__" | _], metadata) do
    format_date(metadata)
  end
  defp parse_metadata([head | tail], metadata) do
    [key, value] = String.split(head, ":", parts: 2, trim: true)
    parse_metadata(tail, Dict.put(metadata, String.to_atom(key), String.strip(value)))
  end

  defp parse_metadata(content) when is_list(content) do
    [slug | content] = content
    parse_metadata(content, %{:slug => slug})
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

    for article <- read_articles(files), do: parse_metadata(article)
  end
end
