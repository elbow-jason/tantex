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
iex(1)> {:ok, index} = Tantex.open("./data/demo", [Tantex.Field.build("name", :text), Tantex.Field.build("age", :u64, stored: true, fast: true)])
{:ok,
 %Tantex.Index{
   __ref__: #Reference<0.670193976.377618434.7366>,
   fields: [
     %Tantex.Field{fast: true, kind: :u64, name: "age", stored: true},
     %Tantex.Field{fast: false, kind: :text, name: "name", stored: true}
   ],
   path: "./data/demo"
 }}
iex(2)> {:ok, id1} = Tantex.insert_documents(index, [%{name: "jason", age: 35}])
{:ok, 1}
iex(3)> {:ok, id2} = Tantex.insert_documents(index, [%{name: "jason goldberger", age: 36}])
{:ok, 2}
iex(4)> {:ok, id3} = Tantex.insert_documents(index, [%{name: "jason louis goldberger", age: 37}])
{:ok, 3}
iex(5)> Tantex.find_one(index, "age", 35){:ok, %{"age" => '#', "name" => ["jason"]}}
iex(6)> Tantex.find_many(index, ["name"], "jason", 2)
{:ok,
 [
   %{"age" => '#', "name" => ["jason"]},
   %{"age" => '$', "name" => ["jason goldberger"]}
 ]}
iex(7)> Tantex.find_many(index, ["name"], "jason", 10)
{:ok,
 [
   %{"age" => '#', "name" => ["jason"]},
   %{"age" => '$', "name" => ["jason goldberger"]},
   %{"age" => '%', "name" => ["jason louis goldberger"]}
 ]}
iex(8)> Tantex.find_one(index, "age", "wrong_type_here")
{:error, {:invalid_field_data, "type: U64 - field_name: \"age\""}}
```
