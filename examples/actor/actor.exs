Application.ensure_all_started(:inets)
Application.ensure_all_started(:httpc)

defmodule Worker do
  def fry_egg(name, caller) do
    {:ok, {_, _, body}} = :httpc.request("http://127.0.0.1:3001/#{name}")
    send(caller, body)
  end
end

caller = self()
eggs = ["A", "B", "C", "D"]

for name <- eggs do
  spawn(Worker, :fry_egg, [name, caller])
end

Enum.each(eggs, fn _ ->
  receive do
    egg -> IO.inspect(egg)
  end
end)
