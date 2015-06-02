defmodule MarcCx.Article do
  use Timex
  alias MarcCx.Article
  alias MarcCx.Markdown

  defstruct title: nil, status: "draft", timestamp: nil, slug: nil, excerpt: nil, html: nil

  defimpl Access, for: Article do
    def get(article, key), do: Map.get(article, key)
    def get_and_update(article, key, fun) do
      value =
        case Map.fetch(article, key) do
          {:ok, value} -> value
          :error -> nil
        end

      {get, update} = fun.(value)
      {get, Map.put(article, key, update)}
    end
  end

  # Public API
  def get_articles do
    get_article_files
    |> read_articles
    |> parse_articles
  end

  def get_article(slug) do
    case get_articles |> Enum.filter(&(&1.slug == slug)) do
      [article] -> {:ok, article}
      [] -> {:error, :notfound}
    end
  end

  def get_article!(slug) do
    {:ok, article} = get_article slug
    article
  end

  def published, do: get_articles |> published
  def published(articles), do: articles |> Enum.filter(&(&1[:status] === "published"))

  def drafts, do: get_articles |> drafts
  def drafts(articles), do: articles |> Enum.filter(&(&1[:status] === "draft"))

  def sort_by(:timestamp, :asc), do: get_articles |> sort_by(:timestamp, :asc)
  def sort_by(:timestamp, :desc), do: get_articles |> sort_by(:timestamp, :desc)

  def sort_by(articles, :timestamp, :asc) do
    articles |> Enum.sort(&(&1.timestamp < &2.timestamp))
  end

  def sort_by(articles, :timestamp, :desc) do
    articles |> Enum.sort(&(&1.timestamp > &2.timestamp))
  end

  # Private API
  defp get_article_files do
    path = Application.app_dir(:marc_cx, "priv/articles")

    path
    |> File.ls!
    |> Enum.map(fn (file) ->
      "#{path}/#{file}"
    end)
  end

  defp read_articles(files, articles \\ [])
  defp read_articles([], articles), do: articles
  defp read_articles([file | files], articles) do
    slug  = Path.basename(file, ".md")
    lines = file |> File.read! |> String.split(~r{\r\n?|\n})

    read_articles(files, articles ++ [[slug | lines]])
  end

  defp parse_articles(articles) do
    Enum.map(articles, fn (article) ->
      [slug | lines] = article

      {metadata, html} = lines |> Markdown.parse

      {:ok, date} = DateFormat.parse(metadata.published, "{M}/{D}/{YYYY}")

      metadata =
        metadata
        |> Map.put(:timestamp, Date.convert(date, :secs))
        |> Map.put(:slug, slug)
        |> Map.put(:html, html)

      struct(Article, metadata)
    end)
  end
end
