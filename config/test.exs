use Mix.Config

# We don't run a server during test. If one is required,
# you can enable the server option below.
config :marc_cx, MarcCx.Endpoint,
  http: [port: 4001],
  server: false

# Print only warnings and errors during test
config :logger, level: :warn

# Configure your database
config :marc_cx, MarcCx.Repo,
  adapter: Ecto.Adapters.Postgres,
  username: "marc",
  password: "",
  database: "marc_cx_test",
  size: 1 # Use a single connection for transactional tests
