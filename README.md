# Tantex

This is an experimental lib for interacting with [tantivy](https://github.com/tantivy-search/tantivy) from Elixir.

## Installation

If [available in Hex](https://hex.pm/docs/publish), the package can be installed
by adding `tantex` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:tantex, "~> 0.1.0"}
  ]
end
```

Documentation can be generated with [ExDoc](https://github.com/elixir-lang/ex_doc)
and published on [HexDocs](https://hexdocs.pm). Once published, the docs can
be found at [https://hexdocs.pm/tantex](https://hexdocs.pm/tantex).


## Example Placeholder 

```elixir
iex(3)> {:ok, index} = Tantex.open("./data/demo", [Tantex.Field.build("name", :text)])
{:ok,
 %Tantex.Index{
   fields: [%Tantex.Field{fast: false, kind: :text, name: "name", stored: true}],
   index_ref: #Reference<0.1902586135.2029125634.14443>,
   schema_ref: #Reference<0.1902586135.2029125634.14442>
 }}
iex(4)> res = Tantex.Index.write_documents(index, [%{name: "jason"}])
{:ok, 3}
iex(5)> Tantex.Index.limit_search(index, [:name], "jason")
{:ok, [%{"name" => ["jason"]}, %{"name" => ["jason"]}, %{"name" => ["jason"]}]}
iex(6)> res = Tantex.Index.write_documents(index, [%{name: "jason goldberger"}])
{:ok, 4}
iex(7)> Tantex.Index.limit_search(index, [:name], "jason AND goldberger")
{:ok, [%{"name" => ["jason goldberger"]}]}
iex(8)>
```
