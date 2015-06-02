defmodule MarcCx.PageView do
  use MarcCx.Web, :view
  use Timex
  alias MarcCx.Article

  def format_date(%Article{:timestamp => timestamp}) do
    {:ok, date} = DateFormat.parse("#{timestamp}", "{s-epoch}")

    [month, day, year] =
      date
      |> DateFormat.format!("%B %d %Y", :strftime)
      |> String.split(" ")

    day = day |> String.lstrip(?0)

    day = case String.last(day) do
      "1" -> "#{day}st"
      "2" -> "#{day}nd"
      "3" -> "#{day}rd"
      _   -> "#{day}th"
    end

    "#{month} #{day} #{year}"
  end
end
