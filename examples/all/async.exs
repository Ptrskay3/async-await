Application.ensure_all_started(:inets)
Application.ensure_all_started(:httpc)

tasks =
  for name <- ["A", "B", "C", "D"] do
    Task.async(fn -> :httpc.request("http://127.0.0.1:3001/#{name}") end)
  end

for {:ok, {_, _, body}} <- Task.await_many(tasks) do
  IO.inspect(body)
end
