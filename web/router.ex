defmodule MarcCx.Router do
  use MarcCx.Web, :router

  pipeline :browser do
    plug :accepts, ["html"]
    plug :fetch_session
    plug :fetch_flash
    plug :protect_from_forgery
  end

  pipeline :api do
    plug :accepts, ["json"]
  end

  scope "/", MarcCx do
    pipe_through :browser # Use the default browser stack

    get "/", PageController, :index
    get "/about", PageController, :about
    get "/article/:slug", ArticleController, :index
  end

  # Other scopes may use custom stacks.
  # scope "/api", MarcCx do
  #   pipe_through :api
  # end
end
