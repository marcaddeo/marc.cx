defmodule MarcCx.Markdown do
  def parse(markdown) when markdown |> is_list  do
    {metadata, markdown} = parse_metadata(markdown)

    html = Earmark.to_html(markdown, %Earmark.Options{
      renderer: MarcCx.HtmlRenderer,
      footnotes: true
    })

    {metadata, html}
  end

  def get_metadata(markdown) when markdown |> is_list do
    {metadata, _} = parse_metadata(markdown)

    metadata
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
  defp parse_metadata(content) when content |> is_list do
    parse_metadata(content, %{})
  end
end
