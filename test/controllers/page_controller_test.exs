defmodule MarcCx.PageControllerTest do
  use MarcCx.ConnCase

  test "GET /" do
    conn = get conn(), "/"
    assert html_response(conn, 200) =~ "marc.cx"
  end
end
