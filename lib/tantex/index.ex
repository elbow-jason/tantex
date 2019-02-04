defmodule Tantex.Index do
  alias Tantex.{Encoding, Native, Index, Field}

  defstruct [:index_ref, :schema_ref, :fields]

  def write_documents(%Index{index_ref: index_ref, schema_ref: schema_ref}, docs, opts \\ [])
      when is_list(docs) do
    heap_size = Keyword.get(opts, :heap_size, 50_000_000)
    list_of_docs = Enum.map(docs, &Encoding.encode_term/1)
    Native.write_documents(schema_ref, index_ref, list_of_docs, heap_size)
  end

  def limit_search(
        %Index{index_ref: index_ref, schema_ref: schema_ref},
        fields,
        search_terms,
        limit \\ 10
      ) do
    fields =
      Enum.map(fields, fn
        x when is_binary(x) -> x
        x when is_atom(x) -> to_string(x)
        %Field{name: name} -> to_string(name)
      end)

    case Native.limit_search(schema_ref, index_ref, fields, search_terms, limit) do
      {:ok, json} ->
        {:ok, Enum.map(json, &Jason.decode!/1)}

      err ->
        err
    end
  end
end
