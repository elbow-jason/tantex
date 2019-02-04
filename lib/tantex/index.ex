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
    fields = Enum.map(fields, &string_field/1)

    case Native.limit_search(schema_ref, index_ref, fields, search_terms, limit) do
      {:ok, json_list} ->
        encoder = Encoder.get_encoder()
        {:ok, Enum.map(json_list, fn item -> encoder.decode_map(item) end)}

      err ->
        err
    end
  end

  def find_one_by_term(
        %Index{index_ref: index_ref, schema_ref: schema_ref},
        field,
        text_term
      )
      when is_binary(text_term) do
    case Native.find_one_by_text(schema_ref, index_ref, string_field(field), text_term) do
      {:ok, json_doc} ->
        {:ok, Encoder.get_encoder().decode_map(json_doc)}

      err ->
        err
    end
  end

  defp string_field(x) when is_binary(x), do: x
  defp string_field(x) when is_atom(x), do: to_string(x)
  defp string_field(%Field{name: name}), do: to_string(name)
end
