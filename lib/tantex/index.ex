defmodule Tantex.Index do
  alias Tantex.{Encoder, Native, Index, Field}

  defstruct [:index_ref, :schema_ref, :fields]

  def write_documents(%Index{index_ref: index_ref, schema_ref: schema_ref}, docs, opts \\ [])
      when is_list(docs) do
    heap_size = Keyword.get(opts, :heap_size, 50_000_000)
    encoder = Encoder.get_encoder()
    list_of_docs = Enum.map(docs, fn doc -> encoder.encode_map(doc) end)
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
      {:ok, json_list} ->
        encoder = Encoder.get_encoder()
        {:ok, Enum.map(json_list, fn item -> encoder.decode_map(item) end)}

      err ->
        err
    end
  end
end
