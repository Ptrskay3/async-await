# Run with: `elixirc sync.exs`.
Application.ensure_all_started(:inets)
Application.ensure_all_started(:httpc)

for name <- ["A", "B", "C", "D"] do
  {:ok, {_, _, body}} = :httpc.request("http://127.0.0.1:3001/#{name}")
  IO.inspect(body)
end
